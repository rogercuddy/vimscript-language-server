// Copyright 2019 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::lexer::TokenType;
use crate::parser::Expression;
use crate::parser::if_statement::IfStatement;
use crate::parser::return_statement::ReturnStatement;
use crate::parser::set_statement::SetStatement;
use crate::parser::try_statement::TryStatement;
use crate::parser::while_statement::WhileStatement;
use serde_json::json;

#[derive(PartialEq, Debug)]
pub enum Statement {
    Let(LetStatement),
    Call(CallStatement),
    Execute(ExecuteStatement),
    Return(ReturnStatement),
    If(IfStatement),
    While(WhileStatement),
    Function(FunctionStatement),
    For(ForStatement),
    Try(TryStatement),
    Set(SetStatement),
    Break(BreakStatement),
}

impl Statement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return match &self {
            Statement::Let(x) => json!({ "let": x.dump_for_testing() }),
            Statement::If(x) => json!({ "if": x.dump_for_testing() }),
            Statement::Call(x) => json!({ "call": x.dump_for_testing() }),
            Statement::Return(x) => json!({ "return": x.dump_for_testing() }),
            Statement::While(x) => json!({ "while": x.dump_for_testing() }),
            Statement::Function(x) => json!({ "function": x.dump_for_testing() }),
            Statement::Try(x) => json!({ "try": x.dump_for_testing() }),
            Statement::Set(x) => json!({ "set": x.dump_for_testing() }),
            Statement::Break(x) => json!({ "break": x.dump_for_testing() }),
            _ => json!({}),
        };
    }
}

#[derive(PartialEq, Debug)]
pub struct LetStatement {
    pub var: Expression,
    pub operator: TokenType,
    pub value: Expression,
}

impl LetStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "var": self.var.dump_for_testing(),
            "operator": self.operator.as_str(),
            "value": self.value.dump_for_testing(),
        });
    }
}

#[derive(PartialEq, Debug)]
pub struct CallStatement {
    pub name: String,
    pub arguments: Vec<Expression>,
}

impl CallStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "method": self.name,
            "arguments": self.arguments.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
        });
    }
}

#[derive(PartialEq, Debug)]
pub struct BreakStatement {}

impl BreakStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({});
    }
}

#[derive(PartialEq, Debug)]
pub struct ExecuteStatement {
    pub arguments: Vec<Expression>,
}

#[derive(PartialEq, Debug)]
pub struct FunctionStatement {
    pub name: String,
    // TODO change to list of tokens?
    pub arguments: Vec<String>,
    pub body: Vec<Statement>,
    // true if 'function!'
    pub overwrite: bool,
    pub abort: bool,
}

impl FunctionStatement {
    pub fn dump_for_testing(&self) -> serde_json::Value {
        return json!({
            "name": self.name,
            "arguments": self.arguments,
            "body": self.body.iter().map(|s| s.dump_for_testing()).collect::<Vec<serde_json::Value>>(),
            "overwrite": self.overwrite,
            "abort": self.abort,
        });
    }
}

#[derive(PartialEq, Debug)]
pub struct ForStatement {
    pub loop_variable: LoopVariable,
    pub range: Expression,
    pub body: Vec<Statement>,
}

#[derive(PartialEq, Debug)]
pub enum LoopVariable {
    Single(String),
    List(Vec<String>),
}