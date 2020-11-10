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

use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub enum Instruction<'i> {
    Null(Null),
    Call(Call<'i>),
    Seq(Seq<'i>),
    Par(Par<'i>),
    Xor(Xor<'i>),
    Fold(Fold<'i>),
    Next(Next<'i>),
    Error,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PeerPart<'i> {
    PeerPk(Value<'i>),
    PeerPkWithServiceId(Value<'i>, Value<'i>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum FunctionPart<'i> {
    FuncName(Value<'i>),
    ServiceIdWithFuncName(Value<'i>, Value<'i>),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Call<'i> {
    pub peer_part: PeerPart<'i>,
    pub function_part: FunctionPart<'i>,
    pub args: Vec<Value<'i>>,
    pub output: CallOutput<'i>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Value<'i> {
    Variable(&'i str),
    Literal(&'i str),
    JsonPath { variable: &'i str, path: &'i str },
    CurrentPeerId,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CallOutput<'i> {
    Scalar(&'i str),
    Accumulator(&'i str),
    None,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Seq<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Debug, PartialEq, Eq)]
pub struct Par<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Debug, PartialEq, Eq)]
pub struct Xor<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Debug, PartialEq, Eq)]
pub struct Fold<'i> {
    pub iterable: &'i str,
    pub iterator: &'i str,
    pub instruction: Rc<Instruction<'i>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Next<'i>(pub &'i str);

#[derive(Debug, PartialEq, Eq)]
pub struct Null;