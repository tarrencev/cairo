use cairo_lang_casm::{builder::{CasmBuilder},  casm_build_extend};

use crate::invocations::{add_input_variables, get_non_fallthrough_statement_id};
use super::{CompiledInvocation, CompiledInvocationBuilder, InvocationError};

pub fn build_roll(
    builder: CompiledInvocationBuilder<'_>,
) -> Result<CompiledInvocation, InvocationError> {
    let failure_handle_statement_id = get_non_fallthrough_statement_id(&builder);
    let [address, caller_address] = builder.try_get_single_cells()?;

    let mut casm_builder = CasmBuilder::default();
    add_input_variables! {casm_builder,
        deref address;
        deref caller_address;
    };

    casm_build_extend! {casm_builder,
        tempvar error_reason;
        hint Roll {address: address, caller_address: caller_address} into {dst: error_reason};
        jump Failure if error_reason != 0;
    };


    Ok(builder.build_from_casm_builder(
        casm_builder,
        [
            ("Fallthrough", &[], None),
            (
                "Failure",
                &[&[error_reason]],
                Some(failure_handle_statement_id),
            ),
        ],
    ))

}