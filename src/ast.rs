/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use std::fmt::Display;

use crate::token::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

// Program
//  : {Statement}
//  ;

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<Statement>,
    pub range: Range,
}

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format_statements(&self.body))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Program(p) => write!(f, "{}", p),
            Node::Statement(s) => write!(f, "{}", s),
            Node::Expression(e) => write!(f, "{}", e),
        }
    }
}

// Statement
//  : EmptyStatement
//  | FunctionDeclaration
//  | Expression
//  ;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    EmptyStatement,
    FunctionDeclaration(FunctionDeclaration),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub params: Vec<(Identifier, Expression)>,
    pub return_type: Identifier,
    pub body: Box<Expression>,
    pub range: Range,
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::FunctionDeclaration(FunctionDeclaration {
                name,
                params,
                return_type,
                body,
                ..
            }) => match &**body {
                Expression::BlockExpression(BlockExpression {
                    body: block_expression,
                    ..
                }) => {
                    write!(
                        f,
                        "function {} ({}) type {} {}\n",
                        name,
                        format_params(params),
                        return_type,
                        format_expressions_with_new_line(block_expression)
                    )
                }
                _ => {
                    write!(
                        f,
                        "function {} ({}) type {} = {}\n",
                        name,
                        format_params(params),
                        return_type,
                        body
                    )
                }
            },
            Statement::Expression(expression) => {
                write!(f, "{}\n", expression)
            }
            Statement::EmptyStatement => {
                write!(f, "")
            }
        }
    }
}

// Expression
//  : BlockExpression
//  | LetExpression
//  | ForExpression
//  | BranchExpression
//  | MatchExpression
//  | IfExpression
//  | BinaryExpression
//  | UnaryExpression
//  | FunctionCallExpression
//  | MemberExpression
//  | ConstructorExpression
//  | Identifier
//  | Literal
//  ;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    BlockExpression(BlockExpression),
    LetExpression(LetExpression),
    ForExpression(ForExpression),
    BranchExpression(BranchExpression),
    MatchExpression(MatchExpression),
    IfExpression(IfExpression),

    // operator with precedence
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    FunctionCallExpression(FunctionCallExpression),
    MemberExpression(MemberExpression),
    ConstructorExpression(ConstructorExpression),

    // primary expression
    Identifier(Identifier),
    Literal(Literal),
}

// 显式代码块 `do...` 表达式，或者隠式的花括号 `{...}` 代码块
#[derive(Debug, Clone, PartialEq)]
pub struct BlockExpression {
    pub is_explicit: bool,
    pub body: Vec<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExpression {
    pub is_match: bool, // false == '=', true == 'match'
    pub object: Box<Expression>,
    pub value: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForExpression {
    pub is_in: bool, // false == 'for...let', true == 'for...in'
    pub object: Box<Expression>,
    pub value: Box<Expression>,
    pub body: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchExpression {
    pub additional: Option<Box<Expression>>,
    pub cases: Vec<BranchCaseFragment>,
    pub default: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchCaseFragment {
    pub condition: Box<Expression>,
    pub additional: Option<Expression>,
    pub body: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpression {
    pub additional: Option<Box<Expression>>,
    pub cases: Vec<MatchCaseFragment>,
    pub default: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCaseFragment {
    pub condition: Box<Expression>,
    pub additional: Vec<MatchCaseAdditional>,
    pub body: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchCaseAdditionalType {
    Where,
    Only,
    In,
    Regular,
    Template,
    Into,
    To,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCaseAdditional {
    pub additional_type: MatchCaseAdditionalType,
    pub body: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequent: Box<Expression>,
    pub alternate: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpression {
    pub operator: Token,
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpression {
    pub operator: Token,
    pub operand: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberExpression {
    pub is_computed: bool, // false == 'foo.bar', true == '[...]'
    pub object: Box<Expression>,
    pub property: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorExpression {
    pub object: Identifier,
    pub value: Map,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub dirs: Vec<Identifier>,
    pub name: String,
    // pub is_prefix: bool, // 是否函数的前置调用格式，即 `name!`
    // pub generic_type: Identifier
    pub range: Range,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut full_path = String::new();
        if self.dirs.len() > 0 {
            let path = self
                .dirs
                .iter()
                .map(|d| d.name.clone())
                .collect::<Vec<String>>()
                .join("::");
            full_path.push_str(&path);
            full_path.push_str("::");
        }
        full_path.push_str(&self.name);
        write!(f, "{}", full_path)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BlockExpression(BlockExpression { body, .. }) => {
                let text = format_expressions_with_new_line(body);
                write!(f, "{{\n{}\n}}", text)
            }
            Expression::LetExpression(LetExpression {
                is_match,
                object,
                value,
                ..
            }) => {
                if *is_match {
                    write!(f, "let {} match {}", object, value)
                } else {
                    write!(f, "let {} = {}", object, value)
                }
            }
            Expression::ForExpression(ForExpression {
                is_in,
                object,
                value,
                body,
                ..
            }) => {
                if *is_in {
                    write!(f, "for let {} in {} {}", object, value, body)
                } else {
                    write!(f, "for let {} = {} {}", object, value, body)
                }
            }
            Expression::BranchExpression(BranchExpression {
                additional,
                cases,
                default,
                ..
            }) => {
                todo!()
            }
            Expression::MatchExpression(MatchExpression {
                additional,
                cases,
                default,
                ..
            }) => {
                todo!()
            }
            Expression::IfExpression(IfExpression {
                condition,
                consequent,
                alternate,
                ..
            }) => match alternate {
                Some(a) => {
                    write!(f, "if {} then {} else {}", condition, consequent, a)
                }
                None => {
                    write!(f, "if {} then {}", condition, consequent)
                }
            },
            Expression::BinaryExpression(BinaryExpression {
                operator,
                left,
                right,
                ..
            }) => {
                write!(f, "({} {} {})", left, operator, right)
            }
            Expression::UnaryExpression(UnaryExpression {
                operator, operand, ..
            }) => {
                write!(f, "({} {})", operator, operand)
            }
            Expression::FunctionCallExpression(FunctionCallExpression {
                callee,
                arguments,
                ..
            }) => {
                write!(
                    f,
                    "{} ({})",
                    callee,
                    format_expressions_with_comma(arguments)
                )
            }
            Expression::MemberExpression(MemberExpression {
                is_computed,
                object,
                property,
                ..
            }) => {
                if *is_computed {
                    write!(f, "{}[{}]", object, property)
                } else {
                    write!(f, "{}.{}", object, property)
                }
            }
            Expression::ConstructorExpression(ConstructorExpression { object, value, .. }) => {
                write!(f, "{} {}", object, value)
            }
            Expression::Identifier(i) => {
                write!(f, "{}", i)
            }
            Expression::Literal(i) => {
                write!(f, "{}", i)
            }
        }
    }
}

// Literal
//  : Integer
//  | Float
//  | Imaginary
//  | Bit
//  | Boolean
//  | Char
//  | GeneralString
//  | TemplateString
//  | HashString
//  | NamedOperator
//  | List
//  | Array
//  | Tuple
//  | Map
//  ;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(Integer),
    Float(Float),
    Imaginary(Imaginary),
    Bit(Bit),
    Boolean(Boolean),
    Char(Char),
    GeneralString(GeneralString),
    TemplateString(TemplateString),
    HashString(HashString),
    NamedOperator(NamedOperator),
    List(List),
    Tuple(Tuple),
    Map(Map),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
    pub value: i64,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    pub value: f64,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Imaginary {
    pub value: f64,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bit {
    pub width: u8,
    pub value: i64,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Boolean {
    pub value: bool,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Char {
    pub value: char,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GeneralString {
    pub value: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TemplateString {
    pub fragments: Vec<GeneralString>,
    pub expressions: Vec<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashString {
    pub value: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedOperator {
    pub value: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub is_array: bool, // false == 'list', true == 'array'
    pub elements: Vec<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
    pub elements: Vec<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub elements: Vec<(Expression, Expression)>,
    pub range: Range,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .elements
            .iter()
            .map(|(name, value)| {
                let mut buf = String::new();
                buf.push_str(&name.to_string());
                buf.push_str(": ");
                buf.push_str(&value.to_string());
                buf
            })
            .collect::<Vec<String>>()
            .join(", ");

        write!(f, "{{{}}}", text)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(Integer { value, .. }) => write!(f, "{}", value),
            Literal::Float(Float { value, .. }) => write!(f, "{}", value),
            Literal::Imaginary(Imaginary { value, .. }) => write!(f, "{}i", value),
            Literal::Bit(Bit { width, value, .. }) => write!(f, "{}'d{}", width, value),
            Literal::Boolean(Boolean { value, .. }) => write!(f, "{}", value),

            Literal::Char(Char { value, .. }) => write!(f, "'{}'", value),
            Literal::GeneralString(GeneralString { value, .. }) => write!(f, "\"{}\"", value),
            Literal::TemplateString(TemplateString {
                fragments,
                expressions,
                ..
            }) => {
                let fragment_text_iter = fragments
                    .iter()
                    .map(|general_string| general_string.value.clone());
                let mut expression_text_iter = expressions.iter().map(|exp| exp.to_string());

                let mut cross_combined = Vec::<String>::new();

                // 交叉合并两个矢量
                for e1 in fragment_text_iter {
                    cross_combined.push(e1);
                    if let Some(e2) = expression_text_iter.next() {
                        cross_combined.push(e2);
                    }
                }

                for e2 in expression_text_iter {
                    cross_combined.push(e2);
                }

                write!(f, "`{}`", cross_combined.join(""))
            }
            Literal::HashString(HashString { value, .. }) => write!(f, "#{}", value),
            Literal::NamedOperator(NamedOperator { value, .. }) => write!(f, ":{}:", value),
            Literal::List(List {
                is_array, elements, ..
            }) => {
                let text = format_expressions_with_comma(elements);
                if *is_array {
                    write!(f, "#[{}]", text)
                } else {
                    write!(f, "[{}]", text)
                }
            }
            Literal::Tuple(Tuple { elements, .. }) => {
                let text = format_expressions_with_comma(elements);
                write!(f, "({})", text)
            }
            Literal::Map(m) => {
                write!(f, "{}", m)
            }
        }
    }
}

fn format_statements(statements: &[Statement]) -> String {
    statements
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("")
}

fn format_params(params: &[(Identifier, Expression)]) -> String {
    params
        .iter()
        .map(|(id, exp)| {
            let mut buf = String::new();
            buf.push_str(&id.to_string());
            buf.push_str(" ");
            buf.push_str(&exp.to_string());
            buf
        })
        .collect::<Vec<String>>()
        .join(", ")
}

fn format_expressions_with_comma(expressions: &[Expression]) -> String {
    expressions
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

fn format_expressions_with_new_line(expressions: &[Expression]) -> String {
    expressions
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join("\n")
}

// 记录 Node 在源文件中的位置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range {
    pub file_id: usize, // 源文件 id
    pub start: usize,   // 开始位置
    pub end: usize,     // 结束位置（不包括）
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, GeneralString, Identifier, NamedOperator},
        token::Token,
    };

    use super::{
        BinaryExpression, Boolean, FunctionDeclaration, IfExpression, Integer, Literal, Node,
        Program, Range, Statement,
    };

    fn new_range() -> Range {
        Range {
            file_id: 0,
            start: 0,
            end: 0,
        }
    }

    #[test]
    fn test_display_literal_integer() {
        let i1 = Literal::Integer(Integer {
            value: 123,
            range: new_range(),
        });

        assert_eq!(i1.to_string(), "123");
    }

    #[test]
    fn test_display_literal_general_string() {
        let i1 = Literal::GeneralString(GeneralString {
            value: "foo".to_string(),
            range: new_range(),
        });

        assert_eq!(i1.to_string(), "\"foo\"");
    }

    #[test]
    fn test_display_literal_named_operator() {
        let i1 = Literal::NamedOperator(NamedOperator {
            value: "foo".to_string(),
            range: new_range(),
        });

        assert_eq!(i1.to_string(), ":foo:");
    }

    #[test]
    fn test_display_expression_literal() {
        let i1 = Literal::Integer(Integer {
            value: 123,
            range: new_range(),
        });

        let e1 = Expression::Literal(i1);

        assert_eq!(e1.to_string(), "123");
    }

    #[test]
    fn test_display_expression_identifier() {
        let e1 = Expression::Identifier(Identifier {
            dirs: vec![],
            name: "User".to_string(),
            range: new_range(),
        });

        assert_eq!(e1.to_string(), "User");

        let e2 = Expression::Identifier(Identifier {
            dirs: vec![
                Identifier {
                    dirs: vec![],
                    name: "User".to_string(),
                    range: new_range(),
                },
                Identifier {
                    dirs: vec![Identifier {
                        dirs: vec![],
                        name: "User".to_string(),
                        range: new_range(),
                    }],
                    name: "Address".to_string(),
                    range: new_range(),
                },
            ],
            name: "City".to_string(),
            range: new_range(),
        });

        assert_eq!(e2.to_string(), "User::Address::City");
    }

    #[test]
    fn test_display_expression_if() {
        let e1 = Expression::IfExpression(IfExpression {
            condition: Box::new(Expression::Literal(Literal::Boolean(Boolean {
                value: true,
                range: new_range(),
            }))),
            consequent: Box::new(Expression::Literal(Literal::Integer(Integer {
                value: 1,
                range: new_range(),
            }))),
            alternate: Some(Box::new(Expression::Literal(Literal::Integer(Integer {
                value: 2,
                range: new_range(),
            })))),
            range: new_range(),
        });

        assert_eq!(e1.to_string(), "if true then 1 else 2")
    }

    #[test]
    fn test_display_function_declaration_statement() {
        let s1 = Statement::FunctionDeclaration(FunctionDeclaration {
            name: "inc".to_string(),
            params: vec![(
                Identifier {
                    dirs: vec![],
                    name: "Int".to_string(),
                    range: new_range(),
                },
                Expression::Identifier(Identifier {
                    dirs: vec![],
                    name: "a".to_string(),
                    range: new_range(),
                }),
            )],
            return_type: Identifier {
                dirs: vec![],
                name: "Int".to_string(),
                range: new_range(),
            },
            body: Box::new(Expression::BinaryExpression(BinaryExpression {
                operator: Token::Plus,
                left: Box::new(Expression::Identifier(Identifier {
                    dirs: vec![],
                    name: "a".to_string(),
                    range: new_range(),
                })),
                right: Box::new(Expression::Literal(Literal::Integer(Integer {
                    value: 1,
                    range: new_range(),
                }))),
                range: new_range(),
            })),
            range: new_range(),
        });

        assert_eq!(s1.to_string(), "function inc (Int a) type Int = (a + 1)\n")
    }

    #[test]
    fn test_display_node_program() {
        let p1 = Node::Program(Program {
            body: vec![Statement::Expression(Expression::Literal(
                Literal::Integer(Integer {
                    value: 123,
                    range: new_range(),
                }),
            ))],
            range: new_range(),
        });

        assert_eq!(p1.to_string(), "123\n");
    }
}
