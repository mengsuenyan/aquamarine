/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::contexts::execution::AValue;
use crate::contexts::execution::ExecutionCtx;
use crate::execution::boxed_value::JValuable;
use crate::execution::ExecutionError;
use crate::execution::ExecutionResult;
use crate::JValue;
use crate::SecurityTetraplet;

use air_parser::ast::CallArgValue;

/// Resolve value to called function arguments.
pub(crate) fn resolve_to_args<'i>(
    value: &CallArgValue<'i>,
    ctx: &ExecutionCtx<'i>,
) -> ExecutionResult<(JValue, Vec<SecurityTetraplet>)> {
    fn handle_string_arg<'i>(arg: &str, ctx: &ExecutionCtx<'i>) -> ExecutionResult<(JValue, Vec<SecurityTetraplet>)> {
        let jvalue = JValue::String(arg.to_string());
        let tetraplet = SecurityTetraplet::literal_tetraplet(ctx.init_peer_id.clone());

        Ok((jvalue, vec![tetraplet]))
    }

    match value {
        CallArgValue::InitPeerId => handle_string_arg(ctx.init_peer_id.as_str(), ctx),
        CallArgValue::Literal(value) => handle_string_arg(value, ctx),
        CallArgValue::Variable(name) => {
            let resolved = resolve_to_jvaluable(name, ctx)?;
            let tetraplets = resolved.as_tetraplets();
            let jvalue = resolved.into_jvalue();

            Ok((jvalue, tetraplets))
        }
        CallArgValue::JsonPath { variable, path } => {
            let resolved = resolve_to_jvaluable(variable, ctx)?;
            let (jvalue, tetraplets) = resolved.apply_json_path_with_tetraplets(path)?;
            let jvalue = jvalue.into_iter().cloned().collect::<Vec<_>>();
            let jvalue = JValue::Array(jvalue);

            Ok((jvalue, tetraplets))
        }
    }
}

/// Constructs jvaluable result from `ExecutionCtx::data_cache` by name.
pub(crate) fn resolve_to_jvaluable<'name, 'i, 'ctx>(
    name: &'name str,
    ctx: &'ctx ExecutionCtx<'i>,
) -> ExecutionResult<Box<dyn JValuable + 'ctx>> {
    use ExecutionError::VariableNotFound;

    let value = ctx
        .data_cache
        .get(name)
        .ok_or_else(|| VariableNotFound(name.to_string()))?;

    match value {
        AValue::JValueRef(value) => Ok(Box::new(value.clone())),
        AValue::JValueAccumulatorRef(acc) => Ok(Box::new(acc.borrow())),
        AValue::JValueFoldCursor(fold_state) => {
            let peeked_value = fold_state.iterable.peek().unwrap();
            Ok(Box::new(peeked_value))
        }
    }
}
