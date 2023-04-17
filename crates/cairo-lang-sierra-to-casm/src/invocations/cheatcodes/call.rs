use std::ops::Deref;

use cairo_lang_casm::builder::CasmBuilder;
use cairo_lang_casm::casm_build_extend;

use super::{CompiledInvocation, CompiledInvocationBuilder, InvocationError};
use crate::invocations::{
    add_input_variables, get_non_fallthrough_statement_id, CostValidationInfo,
};

pub fn build_call(
    builder: CompiledInvocationBuilder<'_>,
) -> Result<CompiledInvocation, InvocationError> {
    let failure_handle_statement_id = get_non_fallthrough_statement_id(&builder);
    let refs = builder.try_get_refs::<3>()?;

    // contract address
    let mut optional_contract_address = None;
    if let [maybe_contract_address] = refs[0].cells.deref() {
        optional_contract_address = Some(maybe_contract_address);
    }
    let contract_address = optional_contract_address.ok_or(InvocationError::InvalidGenericArg)?;

    // function name
    let mut optional_function_name = None;
    if let [maybe_function_name] = refs[1].cells.deref() {
        optional_function_name = Some(maybe_function_name);
    }
    let function_name = optional_function_name.ok_or(InvocationError::InvalidGenericArg)?;

    // calldata
    let [calldata_start, calldata_end] = refs[2].try_unpack()?;

    let mut casm_builder = CasmBuilder::default();
    add_input_variables! {casm_builder,
        deref contract_address;
        deref function_name;
        deref calldata_start;
        deref calldata_end;
    };

    casm_build_extend! {casm_builder,
        tempvar err_code;
        tempvar return_data_start;
        tempvar return_data_end;
        hint Call {
            contract_address: contract_address,
            function_name: function_name,
            calldata_start: calldata_start,
            calldata_end: calldata_end
        } into {
            return_data_start: return_data_start,
            return_data_end: return_data_end,
            err_code: err_code
        };
        ap += 3;
        jump Failure if err_code != 0;
    };

    Ok(builder.build_from_casm_builder(
        casm_builder,
        [
            ("Fallthrough", &[&[return_data_start, return_data_end]], None),
            ("Failure", &[&[err_code]], Some(failure_handle_statement_id)),
        ],
        CostValidationInfo { range_check_info: None, extra_costs: None },
    ))
}
