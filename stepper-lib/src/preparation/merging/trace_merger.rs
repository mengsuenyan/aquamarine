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

use super::MergeCtx;
use super::MergeResult;
use crate::preparation::CallResult;
use crate::preparation::DataMergingError;
use crate::preparation::ExecutedState;
use crate::preparation::ExecutionTrace;
use crate::preparation::ParResult;

use air_parser::ast::Instruction;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TraceMerger<'i> {
    prev_ctx: MergeCtx<'i>,
    current_ctx: MergeCtx<'i>,
    result_trace: ExecutionTrace,
    aqua: &'i Instruction<'i>,
}

impl<'i> TraceMerger<'i> {
    pub(crate) fn new(prev_trace: ExecutionTrace, current_trace: ExecutionTrace, aqua: &'i Instruction<'i>) -> Self {
        let max_trace_len = std::cmp::max(prev_trace.len(), current_trace.len());
        let result_trace = ExecutionTrace::with_capacity(max_trace_len);

        let prev_ctx = MergeCtx::new(prev_trace);
        let current_ctx = MergeCtx::new(current_trace);

        Self {
            prev_ctx,
            current_ctx,
            result_trace,
            aqua,
        }
    }

    pub(crate) fn merge(mut self) -> MergeResult<ExecutionTrace> {
        use crate::log_targets::EXECUTED_TRACE_MERGE;

        self.merge_subtree()?;

        log::trace!(target: EXECUTED_TRACE_MERGE, "merged trace: {:?}", self.result_trace);

        Ok(self.result_trace)
    }

    fn merge_subtree(&mut self) -> MergeResult<()> {
        use DataMergingError::IncompatibleExecutedStates;
        use ExecutedState::*;

        loop {
            let prev_state = self.prev_ctx.next_subtree_state();
            let current_state = self.current_ctx.next_subtree_state();

            match (prev_state, current_state) {
                (Some(Call(prev_call)), Some(Call(call))) => {
                    let resulted_call = Self::merge_call(prev_call, call)?;
                    self.result_trace.push_back(Call(resulted_call));
                }
                (Some(Par(prev_par)), Some(Par(current_par))) => self.merge_par(prev_par, current_par)?,
                (None, Some(s)) => {
                    self.result_trace.push_back(s);

                    let current_states = self.current_ctx.drain_subtree_states()?;
                    self.result_trace.extend(current_states);
                    break;
                }
                (Some(s), None) => {
                    self.result_trace.push_back(s);

                    let prev_states = self.prev_ctx.drain_subtree_states()?;
                    self.result_trace.extend(prev_states);
                    break;
                }
                (None, None) => break,
                // this match arm represents (Call, Par) and (Par, Call) states
                (Some(prev_state), Some(current_state)) => {
                    return Err(IncompatibleExecutedStates(prev_state, current_state))
                }
            }
        }

        Ok(())
    }

    fn merge_call(prev_call_result: CallResult, current_call_result: CallResult) -> MergeResult<CallResult> {
        use CallResult::*;
        use DataMergingError::IncompatibleCallResults;

        match (&prev_call_result, &current_call_result) {
            (CallServiceFailed(prev_err_msg), CallServiceFailed(err_msg)) => {
                if prev_err_msg != err_msg {
                    return Err(IncompatibleCallResults(prev_call_result, current_call_result));
                }
                Ok(current_call_result)
            }
            (RequestSentBy(_), CallServiceFailed(_)) => Ok(current_call_result),
            (CallServiceFailed(_), RequestSentBy(_)) => Ok(prev_call_result),
            (RequestSentBy(prev_sender), RequestSentBy(sender)) => {
                if prev_sender != sender {
                    return Err(IncompatibleCallResults(prev_call_result, current_call_result));
                }

                Ok(prev_call_result)
            }
            (RequestSentBy(_), Executed(..)) => Ok(current_call_result),
            (Executed(..), RequestSentBy(_)) => Ok(prev_call_result),
            (Executed(prev_result), Executed(result)) => {
                if prev_result != result {
                    return Err(IncompatibleCallResults(prev_call_result, current_call_result));
                }

                Ok(prev_call_result)
            }
            (CallServiceFailed(_), Executed(..)) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
            (Executed(..), CallServiceFailed(_)) => Err(IncompatibleCallResults(prev_call_result, current_call_result)),
        }
    }

    fn merge_par(&mut self, prev_par: ParResult, current_par: ParResult) -> MergeResult<()> {
        let prev_subtree_size = self.prev_ctx.subtree_size();
        let current_subtree_size = self.current_ctx.subtree_size();

        let par_position = self.result_trace.len();
        // place a temporary Par value to avoid insertion in the middle
        self.result_trace.push_back(ExecutedState::Par(ParResult(0, 0)));

        let len_before_merge = self.result_trace.len();

        self.prev_ctx.set_subtree_size(prev_par.0);
        self.current_ctx.set_subtree_size(current_par.0);
        self.merge_subtree()?;

        let left_par_size = self.result_trace.len() - len_before_merge;

        self.prev_ctx.set_subtree_size(prev_par.1);
        self.current_ctx.set_subtree_size(current_par.1);
        self.merge_subtree()?;

        let right_par_size = self.result_trace.len() - left_par_size - len_before_merge;

        // update the temporary Par with final values
        self.result_trace[par_position] = ExecutedState::Par(ParResult(left_par_size, right_par_size));

        self.prev_ctx
            .set_subtree_size(prev_subtree_size - prev_par.0 - prev_par.1);
        self.current_ctx
            .set_subtree_size(current_subtree_size - current_par.0 - current_par.1);

        Ok(())
    }
}