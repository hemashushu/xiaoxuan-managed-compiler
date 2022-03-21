/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use std::fmt::{Display, Write};

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
    EmptyStatement, // 当一个程序的有效源码是空的时候，解析而得的语法树就只有一个 EmptyStatement
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
            }) => match body.as_ref() {
                Expression::DoExpression(DoExpression {
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
//  : DoExpression
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
//  | PrefixIdentifier
//  | Ellipsis
//  | Interval
//  | List
//  | Tuple
//  | Map
//  | Literal
//  ;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // general expressions
    DoExpression(DoExpression),
    LetExpression(LetExpression),
    ForExpression(ForExpression),
    BranchExpression(BranchExpression),
    MatchExpression(MatchExpression),
    IfExpression(IfExpression),

    // operating expressions
    BinaryExpression(BinaryExpression),
    UnaryExpression(UnaryExpression),
    FunctionCallExpression(FunctionCallExpression),
    MemberExpression(MemberExpression),
    SliceExpression(SliceExpression),
    ConstructorExpression(ConstructorExpression),

    // primary expressions
    Identifier(Identifier),
    PrefixIdentifier(PrefixIdentifier),
    Ellipsis(Ellipsis),
    Interval(Interval),
    Tuple(Tuple),
    List(List),
    Map(Map),
    Literal(Literal),
}

// 显式代码块 `do...` 表达式，或者隠式的花括号 `{...}` 代码块
#[derive(Debug, Clone, PartialEq)]
pub struct DoExpression {
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
    As,
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
pub struct SliceExpression {
    pub object: Box<Expression>,
    // pub start: Box<Expression>,
    // pub end: Box<Expression>,
    pub interval: Interval,
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
    pub dirs: Vec<String>,
    pub name: String,
    // pub generic_type: Identifier
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrefixIdentifier {
    pub identifier: Identifier,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ellipsis {
    pub name: Option<String>,
    pub range: Range,
}

// interval
//
// e.g.
// [a..b]
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub start: Box<Expression>,
    pub end: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    // pub is_array: bool, // false == 'list', true == 'array'
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
    pub elements: Vec<MapEntry>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapEntry {
    pub key: Box<Expression>,
    pub value: Option<Box<Expression>>,
    pub range: Range,
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.end.as_ref() {
            Some(e) => {
                write!(f, "{}..{}", self.start, e)
            }
            None => write!(f, "{}..", self.start),
        }
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = self
            .elements
            .iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>()
            .join(",\n");

        write!(f, "{{{}}}", text)
    }
}

impl Display for MapEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.value {
            Some(v) => {
                write!(f, "{}: {}", self.key, v)
            }
            None => {
                write!(f, "{}", self.key)
            }
        }
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut full_path = String::new();
        if self.dirs.len() > 0 {
            let path = self.dirs.join("::");
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
            Expression::DoExpression(DoExpression { body, .. }) => {
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
            }) => match operator {
                Token::Cast => {
                    write!(f, "{}^", operand)
                }
                Token::Minus => {
                    write!(f, "-{}", operand)
                }
                Token::Unwrap => {
                    write!(f, "{}?", operand)
                }
                _ => {
                    panic!("unexpected unary operator")
                }
            },
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
            Expression::SliceExpression(SliceExpression {
                object, interval, ..
            }) => {
                write!(f, "{}[{}]", object, interval)
            }
            Expression::ConstructorExpression(ConstructorExpression { object, value, .. }) => {
                write!(f, "{} {}", object, value)
            }
            Expression::Identifier(i) => {
                write!(f, "{}", i)
            }
            Expression::PrefixIdentifier(pi) => {
                write!(f, "!{}", pi.identifier)
            }
            Expression::Ellipsis(Ellipsis { name, .. }) => match name {
                Some(n) => write!(f, "...{}", n),
                None => write!(f, "..."),
            },
            Expression::Interval(i) => {
                write!(f, "{}", i)
            }
            Expression::List(List {
                //is_array,
                elements,
                ..
            }) => {
                let text = format_expressions_with_comma(elements);
                // if *is_array {
                //     write!(f, "#[{}]", text)
                // } else {
                write!(f, "[{}]", text)
                // }
            }
            Expression::Tuple(Tuple { elements, .. }) => {
                let text = format_expressions_with_comma(elements);
                write!(f, "({})", text)
            }
            Expression::Map(m) => {
                write!(f, "{}", m)
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
//  | Complex
//  | Bit
//  | Boolean
//  | Char
//  | GeneralString
//  | TemplateString
//  | HashString
//  | NamedOperator
//  ;

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(Integer),
    Float(Float),
    Complex(Complex),
    Bit(Bit),
    Boolean(Boolean),
    Char(Char),
    GeneralString(GeneralString),
    TemplateString(TemplateString),
    HashString(HashString),
    NamedOperator(NamedOperator),
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
pub struct Complex {
    pub real: f64,
    pub imaginary: f64,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Bit {
    pub bit_width: usize,
    pub bytes: Vec<u8>,
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

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(Integer { value, .. }) => write!(f, "{}", value),
            Literal::Float(Float { value, .. }) => write!(f, "{}", value),
            Literal::Complex(Complex {
                real, imaginary, ..
            }) => write!(f, "{}+{}i", real, imaginary),
            Literal::Bit(Bit {
                bit_width, bytes, ..
            }) => {
                let mut hex = String::new();
                for byte in bytes {
                    write!(hex, "{:02x}", byte)?;
                }
                write!(f, "{}'x{}", bit_width, hex)
            }
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
        ast::{
            Bit, Complex, Expression, GeneralString, Identifier, NamedOperator, PrefixIdentifier,
        },
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

    fn test_display_literal_float() {
        // todo::
    }

    #[test]
    fn test_display_literal_complex() {
        let i1 = Literal::Complex(Complex {
            real: 12.0,
            imaginary: 34.0,
            range: new_range(),
        });

        assert_eq!(i1.to_string(), "12+34i");
    }

    #[test]
    fn test_display_literal_bit() {
        let i1 = Literal::Bit(Bit {
            bit_width: 12,
            bytes: vec![0xab, 0x8, 0x12],
            range: new_range(),
        });

        assert_eq!(i1.to_string(), "12'xab0812");
    }

    fn test_display_literal_boolean() {
        // todo::
    }

    fn test_display_literal_char() {
        // todo::
    }

    #[test]
    fn test_display_literal_general_string() {
        let i1 = Literal::GeneralString(GeneralString {
            value: "foo".to_string(),
            range: new_range(),
        });

        assert_eq!(i1.to_string(), "\"foo\"");
    }

    fn test_display_literal_template_string() {
        // todo::
    }

    fn test_display_literal_hash_string() {
        // todo::
    }

    #[test]
    fn test_display_literal_named_operator() {
        let i1 = Literal::NamedOperator(NamedOperator {
            value: "foo".to_string(),
            range: new_range(),
        });

        assert_eq!(i1.to_string(), ":foo:");
    }

    // primary expressions

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
                // Identifier {
                //     dirs: vec![],
                //     name: "User".to_string(),
                //     range: new_range(),
                // },
                // Identifier {
                //     dirs: vec![Identifier {
                //         dirs: vec![],
                //         name: "User".to_string(),
                //         range: new_range(),
                //     }],
                //     name: "Address".to_string(),
                //     range: new_range(),
                // },
                "User".to_string(),
                "Address".to_string(),
            ],
            name: "City".to_string(),
            range: new_range(),
        });

        assert_eq!(e2.to_string(), "User::Address::City");
    }

    #[test]
    fn test_display_expression_prefix_identifier() {
        let e1 = Expression::PrefixIdentifier(PrefixIdentifier {
            identifier: Identifier {
                dirs: vec![],
                name: "len".to_string(),
                range: new_range(),
            },
            range: new_range(),
        });

        assert_eq!(e1.to_string(), "!len");
    }

    fn test_display_expression_ellipsis() {
        // todo::
    }

    fn test_display_expression_interval() {
        // todo::
    }

    fn test_display_expression_tuple() {
        // todo::
    }

    fn test_display_expression_list() {
        // todo::
    }

    fn test_display_expression_map() {
        // todo::
    }

    // operating expressions

    fn test_display_binary_expression() {
        // todo::
    }

    fn test_display_unary_expression() {
        // todo::
    }

    fn test_display_function_call_expression() {
        // todo::
    }

    fn test_display_member_expression() {
        // todo::
    }

    fn test_display_slice_expression() {
        // todo::
    }

    fn test_display_constructor_expression() {
        // todo::
    }

    // general expressions

    fn test_display_block_expression() {
        // todo::
    }

    fn test_display_let_expression() {
        // todo::
    }

    fn test_display_for_expression() {
        // todo::
    }

    fn test_display_branch_expression() {
        // todo::
    }

    fn test_display_match_expression() {
        // todo::
    }

    #[test]
    fn test_display_if_expression() {
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

    // statements

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

    // nodes

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
