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
//  | NamespaceStatement
//  | UseStatement
//  | ConstDeclaration
//  | StructDeclaration
//  | UnionDeclaration
//  | TraitDeclaration
//  | ImplStatement
//  | AliasStatement
//  | Expression
//  ;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    EmptyStatement, // 当一个程序的有效源码是空的时候，解析而得的语法树就只有一个 EmptyStatement
    FunctionDeclaration(FunctionDeclaration),
    NamespaceStatement(NamespaceStatement),
    UseStatement(UseStatement),
    ConstDeclaration(ConstDeclaration),
    StructDeclaration(StructDeclaration),
    UnionDeclaration(UnionDeclaration),
    TraitDeclaration(TraitDeclaration),
    ImplStatement(ImplStatement),
    AliasStatement(AliasStatement),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    // pub params: Vec<(datatype, name, Expression)>,
    //pub return_type: datatype(sign | Identifier),
    pub body: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceStatement {
    // todo::
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UseStatement {
    // todo::
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDeclaration {
    // todo::
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDeclaration {
    // todo::
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDeclaration {
    // todo::
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDeclaration {
    // todo::
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplStatement {
    // todo::
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AliasStatement {
    pub name: String,
    pub data_type: DataType,
    pub generic_names: Vec<String>,
    pub range: Range,
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        //         match self.body.as_ref() {
        //             Expression::ExpressionBlock(ExpressionBlock {
        //                 body: block_expression,
        //                 ..
        //             }) => {
        //                 write!(
        //                     f,
        //                     "function {} ({}) type {} {}\n",
        //                     self.name,
        //                     format_params(&self.params),
        //                     self.return_type,
        //                     format_expressions_with_new_line(block_expression)
        //                 )
        //             }
        //             _ => {
        //                 write!(
        //                     f,
        //                     "function {} ({}) type {} = {}\n",
        //                     self.name,
        //                     format_params(&self.params),
        //                     self.return_type,
        //                     self.body
        //                 )
        //             }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
        //         match self {
        //             Statement::FunctionDeclaration(declaration) => {
        //                 write!(f, "{}\n", declaration)
        //             },
        //             Statement::Expression(expression) => {
        //                 write!(f, "{}\n", expression)
        //             }
        //             Statement::EmptyStatement => {
        //                 write!(f, "")
        //             }
        //         }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // general expressions
    ExpressionBlock(ExpressionBlock), // `表达式块` 本身也是 `表达式` 其中的一种
    JoinExpression(JoinExpression),
    LetExpression(LetExpression),

    IfExpression(IfExpression),
    ForExpression(ForExpression),
    EachExpression(EachExpression),
    BranchExpression(BranchExpression),
    MatchExpression(MatchExpression),

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

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionBlock {
    // 用于标记是 `do 表达式` 还是 `隠式 do 表达式`。
    //
    // `隠式 do 表达式` 是指省略了 `do` 关键字，
    // 只保留了花括号 `{...}` 的表达式块
    pub is_explicit: bool, // false == `{...}`, true == `do {...}`

    pub body: Vec<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct JoinExpression {
    pub to: Option<Box<Expression>>,
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
pub struct IfExpression {
    pub condition: Box<Expression>,
    pub consequent: Box<Expression>,
    pub alternate: Option<Box<Expression>>, // `else` 部分是可选的
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForExpression {
    pub object: Box<Expression>,
    pub value: Box<Expression>,
    pub body: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EachExpression {
    pub object: Box<Expression>,
    pub value: Box<Expression>,
    pub body: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchExpression {
    pub cases: Vec<BranchCase>,
    pub default: Option<Box<Expression>>,
    pub where_exp: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BranchCase {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
    pub where_exp: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchExpression {
    pub cases: Vec<MatchCase>,
    pub default: Option<Box<Expression>>,
    pub where_exp: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub condition: Box<Expression>,
    pub body: Box<Expression>,
    pub where_exp: Option<Box<Expression>>,
    pub variable: Option<String>, // @ 变量名称
    pub additionals: Vec<MatchCaseAdditional>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchCaseAdditional {
    Only(Expression),             // 附加条件，一个返回 Boolean 值的表达式
    In(Expression),               // 实现了 `Exist` 特性的对象，比如 `Range` 或者 `List`
    Into(String, String),         // 类型名称和标识符名称
    Regular(String, Vec<String>), // 正则表达式字符串字面量，以及变量名列表
    Template(String),             // 带有占位符的字符串字面量
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
    pub arguments: Vec<Argument>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
    pub name: Option<String>,
    pub value: Box<Expression>,
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
    pub interval: Interval,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorExpression {
    pub object: Identifier,
    pub value: Map,
    pub range: Range,
}

// identifier 是局部变量名称或者函数名称
//
// identifier 不包括
// - `let 表达式` 的左值，左值是一个表达式
// - 函数参数列表里的 `参数`
#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub dirs: Vec<String>,
    pub name: String,
    pub generic_names: Vec<String>, // 泛型代号列表
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

// 范围值,用于构建切片和数列
//
// e.g.
// `0..10`
// `0..=9`
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub is_inclusive: bool, // `..` 不包括 `to`，`..=` 包括 `to`
    pub from: Box<Expression>,
    pub to: Option<Box<Expression>>,
    pub range: Range,
}

// 函数的签名
//
// e.g.
// `sign (Int x, Int y) type Int`
// `sign<T, E> (T x, E y) type T`
// `sign (T a, String s) which {T: Int}`
#[derive(Debug, Clone, PartialEq)]
pub struct Sign {
    pub parameters: Vec<Parameter>,
    pub return_data_type: Option<Box<DataType>>,
    pub generic_names: Vec<String>,
    pub which_entries: Vec<WhichEntry>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Sign(Sign),
    Identifier,
}

// 普通函数参数
// 不包括模式函数的参数，模式函数的参数是一个表达式
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub data_type: DataType,
    pub name: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhichEntry {
    pub is_limit: bool, // false == 一般的类型说明，true == 泛型类型约束
    pub name: String,
    pub data_types: Vec<DataType>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
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

impl Display for MatchCaseAdditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MatchCaseAdditional::Only(e) => {
                write!(f, "only {}", e)
            }
            MatchCaseAdditional::In(e) => {
                write!(f, "only {}", e)
            }
        }
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = if self.is_inclusive { "..=" } else { ".." };
        match self.to.as_ref() {
            Some(to_exp) => {
                write!(f, "{}{}{}", self.from, operator, to_exp)
            }
            None => write!(f, "{}{}", self.from, operator),
        }
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.elements.len() == 0 {
            write!(f, "()")
        } else {
            let text = format_expressions_with_comma(&self.elements);
            write!(f, "({},)", text)
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.elements.len() == 0 {
            write!(f, "[]")
        } else {
            let text = format_expressions_with_comma(&self.elements);
            write!(f, "[{},]", text)
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
            Expression::ExpressionBlock(ExpressionBlock { body, .. }) => {
                let text = format_expressions_with_new_line(body);
                write!(f, "{{\n{}\n}}", text)
            }
            Expression::JoinExpression(JoinExpression { to, body, .. }) => match to {
                Some(to_exp) => {
                    write!(
                        f,
                        "join to {} {{\n{}\n}}",
                        to_exp,
                        format_expressions_with_new_line(body)
                    )
                }
                None => {
                    write!(f, "join {{\n{}\n}}", format_expressions_with_new_line(body))
                }
            },
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
                object,
                value,
                body,
                ..
            }) => {
                write!(f, "for let {} = {} {}", object, value, body)
            }
            Expression::EachExpression(EachExpression {
                object,
                value,
                body,
                ..
            }) => {
                write!(f, "each let {} in {} {}", object, value, body)
            }
            Expression::BranchExpression(BranchExpression { cases, default, .. }) => {
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
            Expression::List(l) => {
                write!(f, "{}", l)
            }
            Expression::Tuple(t) => {
                write!(f, "{}", t)
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
    pub width: usize,
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
    pub fragments: Vec<String>,
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
            Literal::Bit(Bit { width, bytes, .. }) => {
                let mut hex = String::new();
                for byte in bytes {
                    write!(hex, "{:02x}", byte)?;
                }
                write!(f, "{}'x{}", width, hex)
            }
            Literal::Boolean(Boolean { value, .. }) => write!(f, "{}", value),

            Literal::Char(Char { value, .. }) => write!(f, "'{}'", value),
            Literal::GeneralString(GeneralString { value, .. }) => write!(f, "\"{}\"", value),
            Literal::TemplateString(TemplateString {
                fragments,
                expressions,
                ..
            }) => {
                let mut expression_text_iter = expressions.iter().map(|exp| exp.to_string());
                let mut cross_combined = Vec::<String>::new();

                // 交叉合并两个列表
                for e1 in fragments {
                    cross_combined.push(e1.to_string());
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

// fn format_params(params: &[(Identifier, Expression)]) -> String {
//     params
//         .iter()
//         .map(|(id, exp)| {
//             let mut buf = String::new();
//             buf.push_str(&id.to_string());
//             buf.push_str(" ");
//             buf.push_str(&exp.to_string());
//             buf
//         })
//         .collect::<Vec<String>>()
//         .join(", ")
// }

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

// 属性，目前仅支持标注在 `namespace` 以及 `function` 语句
//
// e.g.
// `#[name(name1=value1, name2)]`
//
// 其中变量值是可省的
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute {
    pub name: String,                             // 名称
    pub arguments: Vec<(String, Option<String>)>, // 变量名及值
}

// 记录 Node 在源文件中的位置（范围）
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

    #[test]
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
            width: 12,
            bytes: vec![0xab, 0x8, 0x12],
            range: new_range(),
        });

        assert_eq!(i1.to_string(), "12'xab0812");
    }

    #[test]
    fn test_display_literal_boolean() {
        // todo::
    }

    #[test]
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

    #[test]
    fn test_display_literal_template_string() {
        // todo::
    }

    #[test]
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

    #[test]
    fn test_display_expression_ellipsis() {
        // todo::
    }

    #[test]
    fn test_display_expression_interval() {
        // todo::
    }

    #[test]
    fn test_display_expression_tuple() {
        // todo::
    }

    #[test]
    fn test_display_expression_list() {
        // todo::
    }

    #[test]
    fn test_display_expression_map() {
        // todo::
    }

    // operating expressions

    #[test]
    fn test_display_binary_expression() {
        // todo::
    }

    #[test]
    fn test_display_unary_expression() {
        // todo::
    }

    #[test]
    fn test_display_function_call_expression() {
        // todo::
    }

    #[test]
    fn test_display_member_expression() {
        // todo::
    }

    #[test]
    fn test_display_slice_expression() {
        // todo::
    }

    #[test]
    fn test_display_constructor_expression() {
        // todo::
    }

    // general expressions

    #[test]
    fn test_display_block_expression() {
        // todo::
    }

    #[test]
    fn test_display_let_expression() {
        // todo::
    }

    #[test]
    fn test_display_for_expression() {
        // todo::
    }

    #[test]
    fn test_display_branch_expression() {
        // todo::
    }

    #[test]
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
