use std::fs;
use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use cairo_felt::Felt252;
use cairo_lang_compiler::db::RootDatabase;
use cairo_lang_compiler::diagnostics::DiagnosticsReporter;
use cairo_lang_compiler::project::setup_project;
use cairo_lang_debug::DebugWithDb;
use cairo_lang_defs::ids::{FreeFunctionId, FunctionWithBodyId, ModuleItemId};
use cairo_lang_defs::plugin::PluginDiagnostic;
use cairo_lang_diagnostics::ToOption;
use cairo_lang_filesystem::ids::CrateId;
use cairo_lang_lowering::ids::ConcreteFunctionWithBodyId;
use cairo_lang_plugins::get_default_plugins;
use cairo_lang_semantic::db::SemanticGroup;
use cairo_lang_semantic::items::functions::GenericFunctionId;
use cairo_lang_semantic::literals::LiteralLongId;
use cairo_lang_semantic::{ConcreteFunction, FunctionLongId};
use cairo_lang_sierra::extensions::enm::EnumType;
use cairo_lang_sierra::extensions::NamedType;
use cairo_lang_sierra::program::{GenericArg, Program};
use cairo_lang_sierra_generator::db::SierraGenGroup;
use cairo_lang_sierra_generator::replace_ids::replace_sierra_ids_in_program;
use cairo_lang_starknet::plugin::StarkNetPlugin;
use cairo_lang_syntax::attribute::structured::{Attribute, AttributeArg, AttributeArgVariant};
use cairo_lang_syntax::node::db::SyntaxGroup;
use cairo_lang_syntax::node::{ast, Terminal, Token, TypedSyntaxNode};
use cairo_lang_utils::OptionHelper;
use dojo_lang::plugin::DojoPlugin;
use itertools::Itertools;
use unescaper::unescape;
use crate::casm_generator::{SierraCasmGenerator, TestConfig as TestConfigInternal};

/// Expectation for a panic case.
pub enum PanicExpectation {
    /// Accept any panic value.
    Any,
    /// Accept only this specific vector of panics.
    Exact(Vec<Felt252>),
}

/// Expectation for a result of a test.
pub enum TestExpectation {
    /// Running the test should not panic.
    Success,
    /// Running the test should result in a panic.
    Panics(PanicExpectation),
}

/// The configuration for running a single test.
pub struct TestConfig {
    /// The amount of gas the test requested.
    pub available_gas: Option<usize>,
    /// The expected result of the run.
    pub expectation: TestExpectation,
    /// Should the test be ignored.
    pub ignored: bool,
}

/// Finds the tests in the requested crates.
pub fn find_all_tests(
    db: &dyn SemanticGroup,
    main_crates: Vec<CrateId>,
) -> Vec<(FreeFunctionId, TestConfig)> {
    let mut tests = vec![];
    for crate_id in main_crates {
        let modules = db.crate_modules(crate_id);
        for module_id in modules.iter() {
            let Ok(module_items) = db.module_items(*module_id) else {
                continue;
            };
            tests.extend(
                module_items.iter().filter_map(|item| {
                    let ModuleItemId::FreeFunction(func_id) = item else { return None };
                    let Ok(attrs) = db.function_with_body_attributes(FunctionWithBodyId::Free(*func_id)) else { return None };
                    Some((*func_id, try_extract_test_config(db.upcast(), attrs).unwrap()?))
                }),
            );
        }
    }
    tests
}

/// Extracts the configuration of a tests from attributes, or returns the diagnostics if the
/// attributes are set illegally.
pub fn try_extract_test_config(
    db: &dyn SyntaxGroup,
    attrs: Vec<Attribute>,
) -> Result<Option<TestConfig>, Vec<PluginDiagnostic>> {
    let test_attr = attrs.iter().find(|attr| attr.id.as_str() == "test");
    let ignore_attr = attrs.iter().find(|attr| attr.id.as_str() == "ignore");
    let available_gas_attr = attrs.iter().find(|attr| attr.id.as_str() == "available_gas");
    let should_panic_attr = attrs.iter().find(|attr| attr.id.as_str() == "should_panic");
    let mut diagnostics = vec![];
    if let Some(attr) = test_attr {
        if !attr.args.is_empty() {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: attr.id_stable_ptr.untyped(),
                message: "Attribute should not have arguments.".into(),
            });
        }
    } else {
        for attr in [ignore_attr, available_gas_attr, should_panic_attr].into_iter().flatten() {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: attr.id_stable_ptr.untyped(),
                message: "Attribute should only appear on tests.".into(),
            });
        }
    }
    let ignored = if let Some(attr) = ignore_attr {
        if !attr.args.is_empty() {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: attr.id_stable_ptr.untyped(),
                message: "Attribute should not have arguments.".into(),
            });
        }
        true
    } else {
        false
    };
    let available_gas = if let Some(attr) = available_gas_attr {
        if let AttributeArg {
            variant: AttributeArgVariant::Unnamed { value: ast::Expr::Literal(literal), .. },
            ..
        } = &attr.args[0]
        {
            literal.token(db).text(db).parse::<usize>().ok()
        } else {
            diagnostics.push(PluginDiagnostic {
                stable_ptr: attr.id_stable_ptr.untyped(),
                message: "Attribute should have a single value argument.".into(),
            });
            None
        }
    } else {
        None
    };
    let (should_panic, expected_panic_value) = if let Some(attr) = should_panic_attr {
        if attr.args.is_empty() {
            (true, None)
        } else {
            (
                true,
                extract_panic_values(db, attr).on_none(|| {
                    diagnostics.push(PluginDiagnostic {
                        stable_ptr: attr.args_stable_ptr.untyped(),
                        message: "Expected panic must be of the form `expected = <tuple of \
                                  felts>`."
                            .into(),
                    });
                }),
            )
        }
    } else {
        (false, None)
    };
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    Ok(if test_attr.is_none() {
        None
    } else {
        Some(TestConfig {
            available_gas,
            expectation: if should_panic {
                TestExpectation::Panics(if let Some(values) = expected_panic_value {
                    PanicExpectation::Exact(values)
                } else {
                    PanicExpectation::Any
                })
            } else {
                TestExpectation::Success
            },
            ignored,
        })
    })
}

/// Tries to extract the relevant expected panic values.
fn extract_panic_values(db: &dyn SyntaxGroup, attr: &Attribute) -> Option<Vec<Felt252>> {
    let AttributeArg {
        variant: AttributeArgVariant::Unnamed { value: ast::Expr::Binary(binary), .. },
        ..
    } = &attr.args[0] else {
        return None;
    };

    if !matches!(binary.op(db), ast::BinaryOperator::Eq(_)) {
        return None;
    }
    if binary.lhs(db).as_syntax_node().get_text_without_trivia(db) != "expected" {
        return None;
    }
    let ast::Expr::Tuple(panics) = binary.rhs(db) else { return None };
    panics
        .expressions(db)
        .elements(db)
        .into_iter()
        .map(|value| match value {
            ast::Expr::Literal(literal) => {
                Felt252::try_from(LiteralLongId::try_from(literal.token(db).text(db)).ok()?.value).ok()
            }
            ast::Expr::ShortString(short_string_syntax) => {
                let text = short_string_syntax.text(db);
                let (literal, _) = text[1..].rsplit_once('\'')?;
                let unescaped_literal = unescape(literal).ok()?;
                Some(Felt252::from_bytes_be(unescaped_literal.as_bytes()))
            }
            _ => None,
        })
        .collect::<Option<Vec<_>>>()
}

// returns tuple[sierra if no output_path, list[test_name, test_config]]
pub fn collect_tests(
    input_path: &String,
    output_path: Option<&String>,
    maybe_cairo_paths: Option<Vec<&String>>,
    maybe_builtins: Option<Vec<&String>>,
) -> Result<(Option<String>, Vec<TestConfigInternal>)> {
    let mut plugins = get_default_plugins();
    // code taken from crates/cairo-lang-test-runner/src/cli.rs
    plugins.push(Arc::new(DojoPlugin::default()));
    plugins.push(Arc::new(StarkNetPlugin::default()));
    let db = &mut RootDatabase::builder()
        .with_plugins(plugins)
        .detect_corelib()
        .build()
        .context("Failed to build database")?;

    let main_crate_ids = setup_project(db, Path::new(&input_path))
        .with_context(|| format!("Failed to setup project for path({})", input_path))?;

    if let Some(cairo_paths) = maybe_cairo_paths {
        for cairo_path in cairo_paths {
            setup_project(db, Path::new(cairo_path))
                .with_context(|| format!("Failed to add linked library ({})", input_path))?;
        }
    }

    if DiagnosticsReporter::stderr().check(db) {
        return Err(anyhow!(
            "Failed to add linked library, for a detailed information, please go through the logs \
             above"
        ));
    }
    let all_tests = find_all_tests(db, main_crate_ids);

    let sierra_program = db
        .get_sierra_program_for_functions(
            all_tests
                .iter()
                .flat_map(|(func_id, _cfg)| {
                    ConcreteFunctionWithBodyId::from_no_generics_free(db, *func_id)
                })
                .collect(),
        )
        .to_option()
        .context("Compilation failed without any diagnostics")
        .context("Failed to get sierra program")?;

    let collected_tests: Vec<TestConfigInternal> = all_tests
        .into_iter()
        .map(|(func_id, test)| {
            (
                format!(
                    "{:?}",
                    FunctionLongId {
                        function: ConcreteFunction {
                            generic_function: GenericFunctionId::Free(func_id),
                            generic_args: vec![]
                        }
                    }
                    .debug(db)
                ),
                test,
            )
        })
        .collect_vec()
        .into_iter()
        .map(|(test_name, config)| TestConfigInternal {
            name: test_name,
            available_gas: config.available_gas,
        })
        .collect();

    let sierra_program = replace_sierra_ids_in_program(db, &sierra_program);

    let mut builtins = vec![];
    if let Some(unwrapped_builtins) = maybe_builtins {
        builtins = unwrapped_builtins.iter().map(|s| s.to_string()).collect();
    }

    validate_tests(sierra_program.clone(), &collected_tests, builtins)
        .context("Test validation failed")?;

    let mut result_contents = None;
    if let Some(path) = output_path {
        fs::write(path, &sierra_program.to_string()).context("Failed to write output")?;
    } else {
        result_contents = Some(sierra_program.to_string());
    }
    Ok((result_contents, collected_tests))
}

fn validate_tests(
    sierra_program: Program,
    collected_tests: &Vec<TestConfigInternal>,
    ignored_params: Vec<String>,
) -> Result<(), anyhow::Error> {
    let casm_generator = match SierraCasmGenerator::new(sierra_program) {
        Ok(casm_generator) => casm_generator,
        Err(e) => panic!("{}", e),
    };
    for test in collected_tests {
        let func = casm_generator.find_function(&test.name)?;
        let mut filtered_params: Vec<String> = Vec::new();
        for param in &func.params {
            let param_str = &param.ty.debug_name.as_ref().unwrap().to_string();
            if !ignored_params.contains(&param_str) {
                filtered_params.push(param_str.to_string());
            }
        }
        if !filtered_params.is_empty() {
            anyhow::bail!(format!(
                "Invalid number of parameters for test {}: expected 0, got {}",
                test.name,
                func.params.len()
            ));
        }
        let signature = &func.signature;
        let ret_types = &signature.ret_types;
        let tp = &ret_types[ret_types.len() - 1];
        let info = casm_generator.get_info(&tp);
        let mut maybe_return_type_name = None;
        if info.long_id.generic_id == EnumType::ID {
            if let GenericArg::UserType(ut) = &info.long_id.generic_args[0] {
                if let Some(name) = ut.debug_name.as_ref() {
                    maybe_return_type_name = Some(name.as_str());
                }
            }
        }
        if let Some(return_type_name) = maybe_return_type_name {
            if !return_type_name.starts_with("core::PanicResult::") {
                anyhow::bail!("Test function {} must be panicable but it's not", test.name);
            }
            if return_type_name != "core::PanicResult::<((),)>" {
                anyhow::bail!(
                    "Test function {} returns a value {}, it is required that test functions do \
                     not return values",
                    test.name,
                    return_type_name
                );
            }
        } else {
            anyhow::bail!(
                "Couldn't read result type for test function {} possible cause: Test function {} \
                 must be panicable but it's not",
                test.name,
                test.name
            );
        }
    }

    Ok(())
}
