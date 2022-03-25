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
//  | FunctionDeclaration
//  | EmptyFunctionDeclaration
//  | PatternFunctionDeclarationzs
//
//  | NamespaceStatement
//  | UseStatement
//  | ConstDeclaration
//
//  | MemberStructDeclaration
//  | TupleStructDeclaration
//  | EmptyStructDeclaration
//
//  | UnionDeclaration
//  | TraitDeclaration
//  | ImplStatement
//  | AliasStatement
//  | Expression
//  ;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    FunctionDeclaration(FunctionDeclaration),
    EmptyFunctionDeclaration(EmptyFunctionDeclaration), // 空函数
    PatternFunctionDeclarationzs(PatternFunctionDeclaration), // 模式函数

    NamespaceStatement(NamespaceStatement),
    UseStatement(UseStatement),
    ConstDeclaration(ConstDeclaration),

    MemberStructDeclaration(MemberStructDeclaration),
    TupleStructDeclaration(TupleStructDeclaration),
    EmptyStructDeclaration(EmptyStructDeclaration),

    UnionDeclaration(UnionDeclaration),
    TraitDeclaration(TraitDeclaration),
    ImplStatement(ImplStatement),
    AliasStatement(AliasStatement),
    Expression(Expression),
}

// 数据类型包括了：
// - 纯数据的类型，如基本数据类型、用户自定义类型（结构体和联合体）
// - 特性（trait）
// - 函数类型
#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Identifier(Identifier),
    Sign(Sign),
}

// 函数的签名
#[derive(Debug, Clone, PartialEq)]
pub struct Sign {
    pub parameters: Vec<Parameter>,
    pub return_data_type: Option<Box<DataType>>,
    pub generic_names: Vec<String>,
    pub which_entries: Vec<WhichEntry>,
    pub range: Range,
}

// 函数数据类型的补充说明从属表达式
#[derive(Debug, Clone, PartialEq)]
pub struct WhichEntry {
    pub is_limit: bool, // false == 一般的类型说明，true == 泛型类型约束
    pub name: String,
    pub data_types: Vec<DataType>,
    pub range: Range,
}

// 范围值,用于构建切片和数列
//
// e.g.
// `0..10`
// `0..=9`
#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub is_inclusive: bool, // false == `..`（不包括 `to`）， true == `..=` （包括 `to`）
    pub from: Box<Expression>,
    pub to: Option<Box<Expression>>,
    pub range: Range,
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::Identifier(i) => {
                write!(f, "{}", i)
            }
            DataType::Sign(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl Display for Sign {
    // e.g.
    // `sign (Int x, Int y) type Int`
    // `sign <T, E> (T x, E y) type T`
    // `sign (T a, String s) which {T: Int}`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut segments = Vec::<String>::new();

        segments.push("sign".to_string());

        if self.generic_names.len() > 0 {
            //
        }

        todo!()
    }
}

impl Display for WhichEntry {
    // e.g.
    // `T: Int`
    // `T: limit Display`
    // `T: limit Display, Clone`
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut segments = Vec::<String>::new();
        segments.push(format!("{}:", self.name.clone()));

        if self.is_limit {
            segments.push("limit".to_string());
        }

        let mut type_segments = Vec::<String>::new();
        for dt in &self.data_types {
            type_segments.push(dt.to_string())
        }

        let type_text = type_segments.join(", ");
        segments.push(type_text);

        write!(f, "{}", segments.join(" "))
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let operator = if self.is_inclusive { "..=" } else { ".." };
        if let Some(t) = &self.to {
            write!(f, "{}{}{}", &self.from, operator, t)
        } else {
            write!(f, "{}{}", &self.from, operator)
        }
    }
}

// 函数的定义语句
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDeclaration {
    pub name: String,
    pub generic_names: Vec<String>,
    pub parameters: Vec<Parameter>,
    pub return_data_type: Option<DataType>,
    pub which_entries: Vec<WhichEntry>,
    pub where_exp: Option<Expression>,
    pub body: Expression,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyFunctionDeclaration {
    pub name: String,
    pub generic_names: Vec<String>,
    pub parameters: Vec<Parameter>,
    pub return_data_type: Option<DataType>,
    pub which_entries: Vec<WhichEntry>,
    pub where_exp: Option<Expression>,
    pub range: Range,
}

// 普通函数的参数
//
// 注意：
// 模式函数的参数是一个表达式，模式函数的参数不能使用这个 Parameter
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub data_type: DataType,
    pub name: String,
    pub value: Option<Expression>,
    pub range: Range,
}

// 模式函数的定义语句
#[derive(Debug, Clone, PartialEq)]
pub struct PatternFunctionDeclaration {
    pub name: String,
    pub generic_names: Vec<String>,
    pub parameters: Vec<PatternParameter>,
    pub return_data_type: Option<DataType>,
    pub which_entries: Vec<WhichEntry>,
    pub where_exp: Option<Expression>, // 有效范围覆盖各个参数以及整个函数
    pub only_exp: Option<Expression>,  // 在各个参数匹配后，模式函数的最后一道防线
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PatternParameter {
    pub variable: Option<(DataType, String)>, // @ 变量的类型及名称
    pub pattern_exp: Option<Box<Expression>>, // 模式表达式
    pub additionals: Vec<PatternAdditional>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamespaceStatement {
    pub dirs: Vec<String>,
    pub range: Range,
}

// use name
// use name::name::name
// use name::name{one, two, three::baz, four::{foo, bar}}
#[derive(Debug, Clone, PartialEq)]
pub struct UseStatement {
    pub name_path: NamePath,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamePath {
    pub directories: Vec<NamePathItem>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NamePathItem {
    Name(String),
    Children(String, Vec<NamePath>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstDeclaration {
    pub name: String,
    pub value: Expression,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberStructDeclaration {
    pub name: String,
    pub members: Vec<StructMember>,
    pub generic_names: Vec<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TupleStructDeclaration {
    pub name: String,
    pub members: Vec<DataType>,
    pub generic_names: Vec<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyStructDeclaration {
    pub name: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructMember {
    pub data_type: DataType,
    pub name: String,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDeclaration {
    pub members: Vec<UnionMember>,
    pub generic_names: Vec<String>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnionMember {
    Struct(MemberStructDeclaration),
    Tuple(TupleStructDeclaration),
    Empty(EmptyStructDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDeclaration {
    pub name: String,
    pub associated_types: Vec<AssociatedType>, // 关联类型
    pub function_items: Vec<TraitFunctionItem>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssociatedType {
    pub name: String,            // 关联类型名称
    pub object_type: Identifier, // 默认类型
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TraitFunctionItem {
    Function(FunctionDeclaration),
    EmptyFunction(EmptyFunctionDeclaration),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplStatement {
    pub object: Identifier,
    pub inherit: Identifier,                   // 一般是 trait 的名称
    pub associated_types: Vec<AssociatedType>, // 关联类型
    pub which_exp: WhichEntry,
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
        //             Expression::BlockExpression(BlockExpression {
        //                 body: block_expression,
        //                 ..
        //             }) => {
        //                 write!(
        //                     f,
        //                     "function {} ({}) type {} {}\n",
        //                     &self.name,
        //                     format_parameters(&self.params),
        //                     &self.return_type,
        //                     format_expressions_with_new_line(block_expression)
        //                 )
        //             }
        //             _ => {
        //                 write!(
        //                     f,
        //                     "function {} ({}) type {} = {}\n",
        //                     &self.name,
        //                     format_parameters(&self.params),
        //                     &self.return_type,
        //                     &self.body
        //                 )
        //             }
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::FunctionDeclaration(declaration) => {
                write!(f, "{}\n", declaration)
            }
            Statement::Expression(expression) => {
                write!(f, "{}\n", expression)
            }
            _=>{
                todo!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    // general expressions
    BlockExpression(BlockExpression), // `表达式块` 本身也是 `表达式` 其中的一种
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
    AnonymousFunction(AnonymousFunction),
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
pub struct BlockExpression {
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
    pub body: Vec<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetExpression {
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
    pub object: Box<Expression>,
    pub cases: Vec<MatchCase>,
    pub default: Option<Box<Expression>>,
    pub where_exp: Option<Box<Expression>>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCase {
    pub pattern_exp: Option<Box<Expression>>, // 模式表达式
    pub body: Box<Expression>,
    pub variable: Option<String>, // @ 变量名称
    pub additionals: Vec<PatternAdditional>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternAdditional {
    Where(Expression),            // 作用范围仅当前 case （包括 only 从属表达式）有效
    Only(Expression),             // 附加条件，一个返回 Boolean 值的表达式
    In(Expression),               // 实现了 `Exist` 特性的对象，比如 `Range` 或者 `List`
    Into(Identifier, String),     // 类型名称和标识符名称
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

// 函数调用时的参数（实参， argument）
//
// e.g.
// some_func(value1, value2, name1=name_value1, name2=name_value2)
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

// 使用花括号方式的结构体实例化表达式
#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorExpression {
    pub object: Identifier,
    pub value: Map,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousFunction {
    pub parameters: Vec<AnonymousParameter>,
    pub return_data_type: Option<DataType>,
    pub body: Box<Expression>,
    pub range: Range,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousParameter {
    pub data_type: Option<DataType>, // 匿名函数的数据类型允许省略
    pub name: String,
    pub range: Range,
}

// identifier 是局部变量名称或者函数名称（包含名称空间路径）
//
// 注意
// - `let 表达式` 的左值是一个表达式，而不是 identifier
// - 函数参数列表里的参数（形参，parameter）也不是 identifier
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

impl Display for BranchExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut body_lines = Vec::<String>::new();

        for c in &self.cases {
            body_lines.push(format!("{}", c));
        }

        if let Some(e) = &self.default {
            body_lines.push(format!("default: {}", e));
        }

        let body_text = body_lines.join("\n");

        if let Some(w) = &self.where_exp {
            write!(f, "branch where {} {{\n{}\n}}", w, body_text)
        } else {
            write!(f, "branch {{\n{}\n}}", body_text)
        }
    }
}

impl Display for BranchCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut head_line_fragments = Vec::<String>::new();

        head_line_fragments.push(format!("{}", &self.condition));

        if let Some(e) = &self.where_exp {
            head_line_fragments.push(format!("where {}", e));
        }

        let head_text = head_line_fragments.join(" ");
        write!(f, "{}: {}", head_text, &self.body)
    }
}

impl Display for MatchExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut body_lines = Vec::<String>::new();

        for c in &self.cases {
            body_lines.push(format!("{}", c));
        }

        if let Some(e) = &self.default {
            body_lines.push(format!("default: {}", e));
        }

        let body_text = body_lines.join("\n");

        if let Some(w) = &self.where_exp {
            write!(
                f,
                "match {} where {} {{\n{}\n}}",
                &self.object, w, body_text
            )
        } else {
            write!(f, "match {} {{\n{}\n}}", &self.object, body_text)
        }
    }
}

impl Display for MatchCase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first_line_fragments = Vec::<String>::new();

        if let Some(v) = &self.variable {
            first_line_fragments.push(format!("{} @", v));
        }

        if let Some(e) = &self.pattern_exp {
            first_line_fragments.push(format!("{}", e));
        }

        let first_line_text = first_line_fragments.join(" ");

        let mut additional_lines = Vec::<String>::new();

        for a in &self.additionals {
            additional_lines.push(format!("{}", a));
        }

        let additional_text = additional_lines.join("\n");

        let head_text = vec![first_line_text, additional_text].join("\n");
        write!(f, "{}: {}", head_text, &self.body)
    }
}

impl Display for PatternAdditional {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PatternAdditional::Where(e) => {
                write!(f, "where {}", e)
            }
            PatternAdditional::Only(e) => {
                write!(f, "only {}", e)
            }
            PatternAdditional::In(e) => {
                write!(f, "in {}", e)
            }
            PatternAdditional::Into(t, n) => {
                write!(f, "into {} {}", t, n)
            }
            PatternAdditional::Regular(s, names) => {
                write!(f, "regular {} ({})", s, names.join(", "))
            }
            PatternAdditional::Template(s) => {
                write!(f, "template {}", s)
            }
        }
    }
}

impl Display for BlockExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_explicit {
            write!(
                f,
                "do {{\n{}\n}}",
                format_expressions_with_new_line(&self.body)
            )
        } else {
            write!(
                f,
                "{{\n{}\n}}",
                format_expressions_with_new_line(&self.body)
            )
        }
    }
}

impl Display for Tuple {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if (&self.elements).len() == 0 {
            write!(f, "()")
        } else {
            let text = format_expressions_with_comma(&self.elements);
            write!(f, "({},)", text)
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if (&self.elements).len() == 0 {
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
        match &(self.value) {
            Some(v) => {
                write!(f, "{}: {}", &self.key, v)
            }
            None => {
                write!(f, "{}", &self.key)
            }
        }
    }
}

impl Display for AnonymousFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fragments = Vec::<String>::new();

        fragments.push("fn".to_string());
        fragments.push(format!(
            "({})",
            format_anonymous_parameters(&self.parameters)
        ));

        if let Some(dt) = &self.return_data_type {
            fragments.push(format!("type {}", dt));
        }

        match self.body.as_ref() {
            Expression::BlockExpression(e) if e.is_explicit == false => {
                fragments.push(format!("{}", e))
            }
            _ => {
                fragments.push("=".to_string());
                fragments.push(format!("{}", &self.body));
            }
        }

        write!(f, "{}", fragments.join(" "))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut fullname = String::new();

        // 命名空间路径
        if (&self.dirs).len() > 0 {
            let path = (&self.dirs).join("::");
            fullname.push_str(&path);
            fullname.push_str("::");
        }

        // 名称
        fullname.push_str(&self.name);

        // 泛型代号
        if (&self.generic_names).len() > 0 {
            let generic = (&self.generic_names).join(", ");
            fullname.push_str("<");
            fullname.push_str(&generic);
            fullname.push_str(">");
        }

        write!(f, "{}", fullname)
    }
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BlockExpression(e) => {
                write!(f, "{}", e)
            }
            Expression::JoinExpression(JoinExpression { body, .. }) => {
                write!(f, "join {{\n{}\n}}", format_expressions_with_new_line(body))
            }
            Expression::LetExpression(LetExpression { object, value, .. }) => {
                write!(f, "let {} = {}", object, value)
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
            Expression::BranchExpression(e) => {
                write!(f, "{}", e)
            }
            Expression::MatchExpression(e) => {
                write!(f, "{}", e)
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
                write!(f, "{} ({})", callee, format_arguments(arguments))
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
            Expression::AnonymousFunction(l) => {
                write!(f, "{}", l)
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
            Expression::Tuple(t) => {
                write!(f, "{}", t)
            }
            Expression::List(l) => {
                write!(f, "{}", l)
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

// 返回所有 statement.to_string() 的无分隔符拼接
// 注：每个 statement 已经包含换行符，所以拼接时无需分隔符
fn format_statements(statements: &[Statement]) -> String {
    statements
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join("")
}

// 返回普通函数的所有参数以逗号 ", " 的拼接，不包含括号
fn format_parameters(parameters: &[Parameter]) -> String {
    parameters
        .iter()
        .map(|p| format!("{} {}", p.data_type, p.name))
        .collect::<Vec<String>>()
        .join(", ")
}

fn format_generic_names(generic_names: &[String]) -> String {
    generic_names.join(", ")
}

// 返回匿名函数的所有参数以逗号 ", " 的拼接，不包含括号
// 注：匿名函数的参数可省略数据类型
fn format_anonymous_parameters(parameters: &[AnonymousParameter]) -> String {
    parameters
        .iter()
        .map(|p| match &p.data_type {
            Some(dt) => {
                format!("{} {}", dt, p.name)
            }
            None => {
                format!("{}", p.name)
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

// 返回函数调用时所有参数（实参）以逗号 ", " 的拼接，不包含括号
// 注：参数有 "按位置" 和 "按名称" 两种方式。
fn format_arguments(arguments: &[Argument]) -> String {
    arguments
        .iter()
        .map(|a| match &a.name {
            Some(n) => {
                format!("{}={}", n, a.value)
            }
            None => {
                format!("{}", a.value)
            }
        })
        .collect::<Vec<String>>()
        .join(", ")
}

// 返回所有表达式以逗号 ", " 的拼接，不包含括号
fn format_expressions_with_comma(expressions: &[Expression]) -> String {
    expressions
        .iter()
        .map(|e| e.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

// 返回所有表达式以换行符 "\n" 的拼接，不包含括号
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

// 这里仅测试 `字面量` 和 `基本表达式`
// 其他类型节点的测试放在 parser.rs
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
            generic_names: vec![],
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
            generic_names: vec![],
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
                generic_names: vec![],
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
}
