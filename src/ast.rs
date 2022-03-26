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
            _ => {
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
    NextExpression(NextExpression), // `next` 是语句，不过为了简化程序，把它当作表达式
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
pub struct NextExpression {
    pub value: Box<Expression>,
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
    pub parameters: Vec<Identifier>,
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
            segments.push(format!("<{}>", format_generic_names(&self.generic_names)));
        }

        segments.push(format!("({})", format_sign_parameters(&self.parameters)));

        if let Some(dt) = &self.return_data_type {
            segments.push(format!("type {}", dt));
        }

        if self.which_entries.len() > 0 {
            segments.push(format!(
                "which {{\n{}\n}}",
                format_which_entries(&self.which_entries)
            ));
        }

        write!(f, "{}", segments.join(" "))
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
    // 映射表项目的键
    pub key: Box<Expression>,

    // 映射表项目的值
    //
    // 注：
    // - 在使用花括号实例化结构体时，可以使用完整的 `key: value` 表达式
    //   为每个成员赋值，如果当前环境存在一个标识符，名称跟结构体的成员同名，
    //   也可以 `: value` 部分，直接写 `key` 部分，所以 `value` 是可选值。
    // - 当映射表作为 `左手边值` 时，也是可以省略 `value`。
    pub value: Option<Box<Expression>>,
    pub range: Range,
}

impl Display for BlockExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_explicit {
            write!(f, "do {{{}}}", format_expressions_with_new_line(&self.body))
        } else {
            write!(f, "{{{}}}", format_expressions_with_new_line(&self.body))
        }
    }
}

impl Display for JoinExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "join {{{}}}",
            format_expressions_with_new_line(&self.body)
        )
    }
}

impl Display for LetExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "let {} = {}", self.object, self.value)
    }
}

impl Display for IfExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.alternate {
            Some(a) => {
                write!(
                    f,
                    "if {} then {} else {}",
                    self.condition, self.consequent, a
                )
            }
            None => {
                write!(f, "if {} then {}", self.condition, self.consequent)
            }
        }
    }
}

impl Display for ForExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "for let {} = {} {}", self.object, self.value, self.body)
    }
}

impl Display for NextExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "next {}", self.value)
    }
}

impl Display for EachExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "each let {} in {} {}",
            self.object, self.value, self.body
        )
    }
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

impl Display for BinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator, self.right)
    }
}

impl Display for UnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.operator {
            Token::Cast => {
                write!(f, "{}^", self.operand)
            }
            Token::Minus => {
                write!(f, "-{}", self.operand)
            }
            Token::Unwrap => {
                write!(f, "{}?", self.operand)
            }
            _ => {
                panic!("unexpected unary operator")
            }
        }
    }
}

impl Display for FunctionCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}({})", self.callee, format_arguments(&self.arguments))
    }
}

impl Display for MemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_computed {
            write!(f, "{}[{}]", self.object, self.property)
        } else {
            write!(f, "{}.{}", self.object, self.property)
        }
    }
}

impl Display for SliceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.object, self.interval)
    }
}

impl Display for ConstructorExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.object, self.value)
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
        // let mut fullname = String::new();
        let mut fragments = Vec::<String>::new();

        // 命名空间路径
        if self.dirs.len() > 0 {
            fragments.push(self.dirs.join("::"));
            fragments.push("::".to_string());
        }

        // 名称
        fragments.push(self.name.clone());

        // 泛型代号
        if self.generic_names.len() > 0 {
            fragments.push(format!("<{}>", format_generic_names(&self.generic_names)));
        }

        write!(f, "{}", fragments.join(""))
    }
}

impl Display for PrefixIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "!{}", self.identifier)
    }
}

impl Display for Ellipsis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            Some(n) => write!(f, "...{}", n),
            None => write!(f, "..."),
        }
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
            .join("\n"); // 注：映射表项目之间也支持使用逗号分隔

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

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BlockExpression(v) => write!(f, "{}", v),
            Expression::JoinExpression(v) => write!(f, "{}", v),
            Expression::LetExpression(v) => write!(f, "{}", v),
            Expression::IfExpression(v) => write!(f, "{}", v),
            Expression::ForExpression(v) => write!(f, "{}", v),
            Expression::NextExpression(v) => write!(f, "{}", v),
            Expression::EachExpression(v) => write!(f, "{}", v),
            Expression::BranchExpression(v) => write!(f, "{}", v),
            Expression::MatchExpression(v) => write!(f, "{}", v),
            Expression::BinaryExpression(v) => write!(f, "{}", v),
            Expression::UnaryExpression(v) => write!(f, "{}", v),
            Expression::FunctionCallExpression(v) => write!(f, "{}", v),
            Expression::MemberExpression(v) => write!(f, "{}", v),
            Expression::SliceExpression(v) => write!(f, "{}", v),
            Expression::ConstructorExpression(v) => write!(f, "{}", v),
            Expression::AnonymousFunction(v) => write!(f, "{}", v),
            Expression::Identifier(v) => write!(f, "{}", v),
            Expression::PrefixIdentifier(v) => write!(f, "{}", v),
            Expression::Ellipsis(v) => write!(f, "{}", v),
            Expression::Interval(v) => write!(f, "{}", v),
            Expression::Tuple(v) => write!(f, "{}", v),
            Expression::List(v) => write!(f, "{}", v),
            Expression::Map(v) => write!(f, "{}", v),
            Expression::Literal(v) => write!(f, "{}", v),
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

impl Display for Integer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}+{}i", self.real, self.imaginary)
    }
}

impl Display for Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut hex = String::new();
        for byte in &self.bytes {
            write!(hex, "{:02x}", byte)?;
        }
        write!(f, "{}'x{}", self.width, hex)
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'", self.value)
    }
}

impl Display for GeneralString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.value)
    }
}

impl Display for TemplateString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cross_combined_items = Vec::<String>::new();

        // 交叉合并两个列表
        let mut expression_iter = self.expressions.iter();
        for fragment in &self.fragments {
            cross_combined_items.push(fragment.to_string());
            if let Some(expression) = expression_iter.next() {
                cross_combined_items.push(format!("{{{{{}}}}}", expression));
            }
        }

        write!(f, "`{}`", cross_combined_items.join(""))
    }
}

impl Display for HashString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{}", self.value)
    }
}

impl Display for NamedOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, ":{}:", self.value)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(v) => write!(f, "{}", v),
            Literal::Float(v) => write!(f, "{}", v),
            Literal::Complex(v) => write!(f, "{}", v),
            Literal::Bit(v) => write!(f, "{}", v),
            Literal::Boolean(v) => write!(f, "{}", v),

            Literal::Char(v) => write!(f, "{}", v),
            Literal::GeneralString(v) => write!(f, "{}", v),
            Literal::TemplateString(v) => write!(f, "{}", v),
            Literal::HashString(v) => write!(f, "{}", v),
            Literal::NamedOperator(v) => write!(f, "{}", v),
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

// 返回函数签名的所有参数以逗号 ", " 的拼接，不包含括号
fn format_sign_parameters(identifiers: &[Identifier]) -> String {
    identifiers
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(", ")
}

// 返回普通函数的所有参数以逗号 ", " 的拼接，不包含括号
fn format_parameters(parameters: &[Parameter]) -> String {
    parameters
        .iter()
        .map(|p| format!("{} {}", p.data_type, p.name))
        .collect::<Vec<String>>()
        .join(", ")
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

fn format_generic_names(generic_names: &[String]) -> String {
    generic_names.join(", ")
}

fn format_which_entries(which_entries: &[WhichEntry]) -> String {
    which_entries
        .iter()
        .map(|w| w.to_string())
        .collect::<Vec<String>>()
        .join("\n")
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
            Argument, Bit, Complex, Ellipsis, Expression, FunctionCallExpression, GeneralString,
            HashString, Identifier, JoinExpression, List, NamedOperator, PrefixIdentifier,
            UnaryExpression, WhichEntry,
        },
        token::Token,
    };

    use super::{
        BinaryExpression, BlockExpression, Boolean, Char, ConstructorExpression, DataType, Float,
        FunctionDeclaration, IfExpression, Integer, Interval, LetExpression, Literal, Map,
        MapEntry, MemberExpression, Node, Program, Range, Sign, SliceExpression, Statement,
        TemplateString, Tuple,
    };

    // 辅助函数

    fn new_range() -> Range {
        Range {
            file_id: 0,
            start: 0,
            end: 0,
        }
    }

    fn new_identifier(name: &str) -> Identifier {
        Identifier {
            dirs: vec![],
            generic_names: vec![],
            name: name.to_string(),
            range: new_range(),
        }
    }

    fn new_literal_integer(value: i64) -> Literal {
        Literal::Integer(Integer {
            value: value,
            range: new_range(),
        })
    }

    fn new_literal_string(value: &str) -> Literal {
        Literal::GeneralString(GeneralString {
            value: value.to_string(),
            range: new_range(),
        })
    }

    fn new_literal_boolean(value: bool) -> Literal {
        Literal::Boolean(Boolean {
            value: value,
            range: new_range(),
        })
    }

    fn new_tuple(numbers: &[i64]) -> Tuple {
        let mut elements = Vec::<Expression>::new();
        for n in numbers {
            elements.push(Expression::Literal(new_literal_integer(*n)));
        }
        Tuple {
            elements,
            range: new_range(),
        }
    }

    fn new_list(numbers: &[i64]) -> List {
        let mut elements = Vec::<Expression>::new();
        for n in numbers {
            elements.push(Expression::Literal(new_literal_integer(*n)));
        }
        List {
            elements,
            range: new_range(),
        }
    }

    fn new_addition_expression(left: i64, right: i64) -> Expression {
        Expression::BinaryExpression(BinaryExpression {
            left: Box::new(Expression::Literal(new_literal_integer(left))),
            right: Box::new(Expression::Literal(new_literal_integer(right))),
            operator: Token::Plus,
            range: new_range(),
        })
    }

    fn trim_left_margin(s: &str) -> String {
        s.split("\n")
            .map(|s| s.trim_start().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn test_display_integer() {
        let i1 = Integer {
            value: 123,
            range: new_range(),
        };
        assert_eq!(i1.to_string(), "123");
    }

    #[test]
    fn test_display_float() {
        let f1 = Float {
            value: 0.618,
            range: new_range(),
        };
        assert_eq!(f1.to_string(), "0.618");
    }

    #[test]
    fn test_display_complex() {
        let i1 = Complex {
            real: 12.0,
            imaginary: 34.0,
            range: new_range(),
        };
        assert_eq!(i1.to_string(), "12+34i");
    }

    #[test]
    fn test_display_bit() {
        let i1 = Bit {
            width: 12,
            bytes: vec![0xab, 0x8, 0x12],
            range: new_range(),
        };
        assert_eq!(i1.to_string(), "12'xab0812");
    }

    #[test]
    fn test_display_boolean() {
        let b1 = Boolean {
            value: true,
            range: new_range(),
        };
        assert_eq!(b1.to_string(), "true");

        let b2 = Boolean {
            value: false,
            range: new_range(),
        };
        assert_eq!(b2.to_string(), "false");
    }

    #[test]
    fn test_display_char() {
        let c1 = Char {
            value: 'a',
            range: new_range(),
        };
        assert_eq!(c1.to_string(), "'a'");

        let c2 = Char {
            value: '文',
            range: new_range(),
        };
        assert_eq!(c2.to_string(), "'文'");
    }

    #[test]
    fn test_display_general_string() {
        let s1 = GeneralString {
            value: "foo".to_string(),
            range: new_range(),
        };
        assert_eq!(s1.to_string(), "\"foo\"");

        let s2 = GeneralString {
            value: "fo'ob`ar".to_string(),
            range: new_range(),
        };
        assert_eq!(s2.to_string(), "\"fo'ob`ar\"");

        let s3 = GeneralString {
            value: "foo\n  bar".to_string(),
            range: new_range(),
        };
        assert_eq!(s3.to_string(), "\"foo\n  bar\"");
    }

    #[test]
    fn test_display_template_string() {
        let l1 = TemplateString {
            fragments: vec!["foo".to_string(), "bar".to_string(), "baz".to_string()],
            expressions: vec![
                Expression::Identifier(Identifier {
                    dirs: vec![],
                    generic_names: vec![],
                    name: "name".to_string(),
                    range: new_range(),
                }),
                Expression::Identifier(Identifier {
                    dirs: vec![],
                    generic_names: vec![],
                    name: "number".to_string(),
                    range: new_range(),
                }),
            ],
            range: new_range(),
        };

        assert_eq!(l1.to_string(), "`foo{{name}}bar{{number}}baz`");
    }

    #[test]
    fn test_display_hash_string() {
        let h1 = HashString {
            value: "foo".to_string(),
            range: new_range(),
        };
        assert_eq!(h1.to_string(), "#foo");

        let h2 = HashString {
            value: "foo_bar2".to_string(),
            range: new_range(),
        };
        assert_eq!(h2.to_string(), "#foo_bar2");
    }

    #[test]
    fn test_display_named_operator() {
        let i1 = NamedOperator {
            value: "foo".to_string(),
            range: new_range(),
        };

        assert_eq!(i1.to_string(), ":foo:");
    }

    // literal

    #[test]
    fn test_display_literal() {
        // 因为 Literal 的 Display 只是直接返回各种字面量的 Display 结果，
        // 所以不需要再次测试。
        //
        // 这里只测试其中的几种字面量的辅助函数。

        assert_eq!(new_literal_integer(456).to_string(), "456");
        assert_eq!(new_literal_boolean(false).to_string(), "false");
        assert_eq!(new_literal_boolean(true).to_string(), "true");
        assert_eq!(new_literal_string("hello").to_string(), "\"hello\"");
    }

    // primary expressions

    #[test]
    fn test_display_expression_literal() {
        // 因为 Expression::Literal 的 Display 只是直接返回 Literal 的 Display 结果，
        // 所以不需要再次测试。

        let e1 = Expression::Literal(new_literal_integer(123));
        assert_eq!(e1.to_string(), "123");
    }

    #[test]
    fn test_display_identifier() {
        let e1 = Identifier {
            dirs: vec![],
            name: "User".to_string(),
            generic_names: vec![],
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "User");

        // 测试命名空间路径
        let e2 = Identifier {
            dirs: vec!["User".to_string(), "Address".to_string()],
            name: "City".to_string(),
            generic_names: vec![],
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "User::Address::City");

        // 测试泛型
        let e3 = Identifier {
            dirs: vec!["Collection".to_string()],
            name: "LinkList".to_string(),
            generic_names: vec!["String".to_string()],
            range: new_range(),
        };
        assert_eq!(e3.to_string(), "Collection::LinkList<String>");

        // 测试多泛型
        let e4 = Identifier {
            dirs: vec![],
            name: "Result".to_string(),
            generic_names: vec!["T".to_string(), "E".to_string()],
            range: new_range(),
        };
        assert_eq!(e4.to_string(), "Result<T, E>");

        // 测试辅助函数
        let e5 = new_identifier("foo");
        assert_eq!(e5.to_string(), "foo");
    }

    #[test]
    fn test_display_prefix_identifier() {
        let p1 = PrefixIdentifier {
            identifier: new_identifier("len"),
            range: new_range(),
        };
        assert_eq!(p1.to_string(), "!len");
    }

    #[test]
    fn test_display_ellipsis() {
        // 带名称的省略符
        let e1 = Ellipsis {
            name: Some("rest".to_string()),
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "...rest");

        // 不带名称的省略符
        let e2 = Ellipsis {
            name: None,
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "...");
    }

    #[test]
    fn test_display_data_type() {
        let d1 = DataType::Identifier(new_identifier("Point"));
        assert_eq!(d1.to_string(), "Point");

        // 带命名空间路径和泛型的 DataType
        let d2 = DataType::Identifier(Identifier {
            dirs: vec!["Shape".to_string()],
            generic_names: vec!["Int".to_string()],
            name: "Point".to_string(),
            range: new_range(),
        });
        assert_eq!(d2.to_string(), "Shape::Point<Int>");
    }

    #[test]
    fn test_display_sign() {
        let s1 = Sign {
            parameters: vec![new_identifier("Int"), new_identifier("Boolean")],
            return_data_type: Some(Box::new(DataType::Identifier(new_identifier("Int")))),
            which_entries: vec![],
            generic_names: vec![],
            range: new_range(),
        };
        assert_eq!(s1.to_string(), "sign (Int, Boolean) type Int");

        // 无参数、无返回值的函数签名
        let s2 = Sign {
            parameters: vec![],
            return_data_type: None,
            which_entries: vec![],
            generic_names: vec![],
            range: new_range(),
        };
        assert_eq!(s2.to_string(), "sign ()");

        // 带泛型的函数签名
        let s3 = Sign {
            parameters: vec![
                Identifier {
                    dirs: vec![],
                    generic_names: vec!["T".to_string()],
                    name: "List".to_string(),
                    range: new_range(),
                },
                new_identifier("U"),
                new_identifier("Int"),
            ],
            return_data_type: Some(Box::new(DataType::Identifier(Identifier {
                dirs: vec![],
                generic_names: vec!["U".to_string()],
                name: "List".to_string(),
                range: new_range(),
            }))),
            which_entries: vec![],
            generic_names: vec!["T".to_string(), "U".to_string()],
            range: new_range(),
        };
        assert_eq!(s3.to_string(), "sign <T, U> (List<T>, U, Int) type List<U>");
    }

    #[test]
    fn test_display_which() {
        // 单独一个类型说明
        let s1 = Sign {
            parameters: vec![Identifier {
                dirs: vec![],
                generic_names: vec!["T".to_string()],
                name: "List".to_string(),
                range: new_range(),
            }],
            return_data_type: None,
            generic_names: vec!["T".to_string()],
            which_entries: vec![WhichEntry {
                is_limit: false,
                name: "T".to_string(),
                data_types: vec![DataType::Identifier(Identifier {
                    dirs: vec![],
                    generic_names: vec![],
                    name: "Int".to_string(),
                    range: new_range(),
                })],
                range: new_range(),
            }],
            range: new_range(),
        };
        assert_eq!(s1.to_string(), "sign <T> (List<T>) which {\nT: Int\n}");

        // 多个类型说明
        let s2 = Sign {
            parameters: vec![Identifier {
                dirs: vec![],
                generic_names: vec!["T".to_string(), "E".to_string()],
                name: "Result".to_string(),
                range: new_range(),
            }],
            return_data_type: None,
            generic_names: vec!["T".to_string(), "E".to_string()],
            which_entries: vec![
                WhichEntry {
                    is_limit: false,
                    name: "T".to_string(),
                    data_types: vec![DataType::Identifier(Identifier {
                        dirs: vec![],
                        generic_names: vec![],
                        name: "Int".to_string(),
                        range: new_range(),
                    })],
                    range: new_range(),
                },
                WhichEntry {
                    is_limit: false,
                    name: "E".to_string(),
                    data_types: vec![DataType::Identifier(Identifier {
                        dirs: vec![],
                        generic_names: vec![],
                        name: "Error".to_string(),
                        range: new_range(),
                    })],
                    range: new_range(),
                },
            ],
            range: new_range(),
        };
        assert_eq!(
            s2.to_string(),
            "sign <T, E> (Result<T, E>) which {\nT: Int\nE: Error\n}"
        );

        // 类型为函数型，以及带 limit 的类型说明
        let s3 = Sign {
            parameters: vec![
                Identifier {
                    dirs: vec![],
                    generic_names: vec![],
                    name: "F".to_string(),
                    range: new_range(),
                },
                Identifier {
                    dirs: vec![],
                    generic_names: vec!["T".to_string()],
                    name: "List".to_string(),
                    range: new_range(),
                },
            ],
            return_data_type: None,
            generic_names: vec!["T".to_string()],
            which_entries: vec![
                WhichEntry {
                    is_limit: false,
                    name: "F".to_string(),
                    data_types: vec![DataType::Sign(Sign {
                        parameters: vec![
                            Identifier {
                                dirs: vec![],
                                generic_names: vec![],
                                name: "Int".to_string(),
                                range: new_range(),
                            },
                            Identifier {
                                dirs: vec![],
                                generic_names: vec![],
                                name: "Boolean".to_string(),
                                range: new_range(),
                            },
                        ],
                        return_data_type: Some(Box::new(DataType::Identifier(Identifier {
                            dirs: vec![],
                            generic_names: vec![],
                            name: "Int".to_string(),
                            range: new_range(),
                        }))),
                        which_entries: vec![],
                        generic_names: vec![],
                        range: new_range(),
                    })],
                    range: new_range(),
                },
                WhichEntry {
                    is_limit: true,
                    name: "T".to_string(),
                    data_types: vec![
                        DataType::Identifier(Identifier {
                            dirs: vec![],
                            generic_names: vec![],
                            name: "Eq".to_string(),
                            range: new_range(),
                        }),
                        DataType::Identifier(Identifier {
                            dirs: vec![],
                            generic_names: vec![],
                            name: "Display".to_string(),
                            range: new_range(),
                        }),
                    ],
                    range: new_range(),
                },
            ],
            range: new_range(),
        };
        assert_eq!(
            s3.to_string(),
            trim_left_margin(
                "sign <T> (F, List<T>) which {
                F: sign (Int, Boolean) type Int
                T: limit Eq, Display
                }"
            )
        );
    }

    #[test]
    fn test_display_interval() {
        let i1 = Interval {
            is_inclusive: false,
            from: Box::new(Expression::Literal(new_literal_integer(1))),
            to: Some(Box::new(Expression::Literal(new_literal_integer(10)))),
            range: new_range(),
        };
        assert_eq!(i1.to_string(), "1..10");

        // 省略 `to` 值
        let i2 = Interval {
            is_inclusive: false,
            from: Box::new(Expression::Literal(new_literal_integer(1))),
            to: None,
            range: new_range(),
        };
        assert_eq!(i2.to_string(), "1..");

        // `to` 值为闭区间
        let i3 = Interval {
            is_inclusive: true,
            from: Box::new(Expression::Literal(new_literal_integer(1))),
            to: Some(Box::new(Expression::Literal(new_literal_integer(10)))),
            range: new_range(),
        };
        assert_eq!(i3.to_string(), "1..=10");
    }

    #[test]
    fn test_display_tuple() {
        let t1 = Tuple {
            elements: vec![
                Expression::Literal(new_literal_integer(123)),
                Expression::Literal(new_literal_integer(456)),
            ],
            range: new_range(),
        };
        assert_eq!(t1.to_string(), "(123, 456,)");

        // 单一个元素的元组
        let t2 = Tuple {
            elements: vec![Expression::Identifier(new_identifier("abc"))],
            range: new_range(),
        };
        assert_eq!(t2.to_string(), "(abc,)");

        // 空元组
        let t3 = Tuple {
            elements: vec![],
            range: new_range(),
        };
        assert_eq!(t3.to_string(), "()");

        // 嵌套元组
        let t4 = Tuple {
            elements: vec![
                Expression::Literal(new_literal_integer(123)),
                Expression::Literal(new_literal_integer(456)),
                Expression::Tuple(Tuple {
                    elements: vec![
                        Expression::Identifier(new_identifier("abc")),
                        Expression::Identifier(new_identifier("def")),
                    ],
                    range: new_range(),
                }),
            ],
            range: new_range(),
        };
        assert_eq!(t4.to_string(), "(123, 456, (abc, def,),)");

        // 测试辅助函数
        let t5 = new_tuple(&vec![8, 13, 21, 34]);
        assert_eq!(t5.to_string(), "(8, 13, 21, 34,)");
    }

    #[test]
    fn test_display_list() {
        let l1 = List {
            elements: vec![
                Expression::Literal(new_literal_integer(123)),
                Expression::Literal(new_literal_integer(456)),
            ],
            range: new_range(),
        };
        assert_eq!(l1.to_string(), "[123, 456,]");

        // 单一个元素的列表
        let l2 = List {
            elements: vec![Expression::Identifier(new_identifier("abc"))],
            range: new_range(),
        };
        assert_eq!(l2.to_string(), "[abc,]");

        // 空列表
        let l3 = List {
            elements: vec![],
            range: new_range(),
        };
        assert_eq!(l3.to_string(), "[]");

        // 嵌套列表
        let l4 = List {
            elements: vec![
                Expression::Literal(new_literal_integer(123)),
                Expression::Literal(new_literal_integer(456)),
                Expression::List(List {
                    elements: vec![
                        Expression::Identifier(new_identifier("abc")),
                        Expression::Identifier(new_identifier("def")),
                    ],
                    range: new_range(),
                }),
            ],
            range: new_range(),
        };
        assert_eq!(l4.to_string(), "[123, 456, [abc, def,],]");

        // 数列
        let l5 = List {
            elements: vec![Expression::Interval(Interval {
                is_inclusive: false,
                from: Box::new(Expression::Literal(new_literal_integer(1))),
                to: Some(Box::new(Expression::Literal(new_literal_integer(10)))),
                range: new_range(),
            })],
            range: new_range(),
        };
        assert_eq!(l5.to_string(), "[1..10,]");

        // 等差数列
        let l6 = List {
            elements: vec![
                Expression::Literal(new_literal_integer(1)),
                Expression::Interval(Interval {
                    is_inclusive: true,
                    from: Box::new(Expression::Literal(new_literal_integer(3))),
                    to: Some(Box::new(Expression::Literal(new_literal_integer(9)))),
                    range: new_range(),
                }),
            ],
            range: new_range(),
        };
        assert_eq!(l6.to_string(), "[1, 3..=9,]");

        // 省略表达式
        let l7 = List {
            elements: vec![
                Expression::Literal(new_literal_integer(1)),
                Expression::Literal(new_literal_integer(2)),
                Expression::Ellipsis(Ellipsis {
                    name: Some("rest".to_string()),
                    range: new_range(),
                }),
            ],
            range: new_range(),
        };
        assert_eq!(l7.to_string(), "[1, 2, ...rest,]");

        // 检查辅助函数
        let l8 = new_list(&vec![3, 5, 8, 13, 21]);
        assert_eq!(l8.to_string(), "[3, 5, 8, 13, 21,]");
    }

    #[test]
    fn test_display_map() {
        let m1 = Map {
            elements: vec![
                MapEntry {
                    key: Box::new(Expression::Literal(Literal::HashString(HashString {
                        value: "name".to_string(),
                        range: new_range(),
                    }))),
                    value: Some(Box::new(Expression::Literal(new_literal_string("foo")))),
                    range: new_range(),
                },
                MapEntry {
                    key: Box::new(Expression::Identifier(new_identifier("id"))),
                    value: Some(Box::new(Expression::Literal(new_literal_integer(123)))),
                    range: new_range(),
                },
            ],
            range: new_range(),
        };
        assert_eq!(
            m1.to_string(),
            trim_left_margin(
                "{#name: \"foo\"
                id: 123}"
            )
        );

        // 单 `key` 和 `省略号` 的映射表
        let m2 = Map {
            elements: vec![
                MapEntry {
                    key: Box::new(Expression::Identifier(new_identifier("name"))),
                    value: Some(Box::new(Expression::Literal(new_literal_string("bar")))),
                    range: new_range(),
                },
                MapEntry {
                    key: Box::new(Expression::Identifier(new_identifier("id"))),
                    value: None,
                    range: new_range(),
                },
                MapEntry {
                    key: Box::new(Expression::Ellipsis(Ellipsis {
                        name: None,
                        range: new_range(),
                    })),
                    value: None,
                    range: new_range(),
                },
            ],
            range: new_range(),
        };
        assert_eq!(
            m2.to_string(),
            trim_left_margin(
                "{name: \"bar\"
                id
                ...}"
            )
        );
    }

    // operating expressions

    #[test]
    fn test_binary_expression() {
        let e1 = BinaryExpression {
            operator: Token::Plus,
            left: Box::new(Expression::Literal(new_literal_integer(1))),
            right: Box::new(Expression::Literal(new_literal_integer(2))),
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "(1 + 2)");

        // 测试二元运算嵌套
        let e2 = BinaryExpression {
            operator: Token::Asterisk,
            left: Box::new(Expression::Literal(new_literal_integer(1))),
            right: Box::new(Expression::BinaryExpression(BinaryExpression {
                operator: Token::Minus,
                left: Box::new(Expression::Literal(new_literal_integer(2))),
                right: Box::new(Expression::Literal(new_literal_integer(3))),
                range: new_range(),
            })),
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "(1 * (2 - 3))");

        // 测试辅助方法 new_addition_expression
        let e3 = new_addition_expression(11, 22);
        assert_eq!(e3.to_string(), "(11 + 22)");
    }

    #[test]
    fn test_unary_expression() {
        // 测试 `^` 运算
        let e1 = UnaryExpression {
            operator: Token::Cast,
            operand: Box::new(Expression::Identifier(new_identifier("foo"))),
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "foo^");

        // 测试表达式的 `^` 运算
        let e2 = UnaryExpression {
            operator: Token::Cast,
            operand: Box::new(Expression::Tuple(Tuple {
                elements: vec![
                    Expression::Literal(new_literal_integer(123)),
                    Expression::Literal(new_literal_integer(456)),
                ],
                range: new_range(),
            })),
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "(123, 456,)^");

        // 测试 `-` 运算
        let e3 = UnaryExpression {
            operator: Token::Minus,
            operand: Box::new(Expression::Literal(new_literal_integer(1))),
            range: new_range(),
        };
        assert_eq!(e3.to_string(), "-1");

        // 测试表达式的 `-` 运算
        let e4 = UnaryExpression {
            operator: Token::Minus,
            operand: Box::new(new_addition_expression(1, 2)),
            range: new_range(),
        };
        assert_eq!(e4.to_string(), "-(1 + 2)");

        // 测试 `?` 运算
        let e5 = UnaryExpression {
            operator: Token::Unwrap,
            operand: Box::new(Expression::Identifier(new_identifier("foo"))),
            range: new_range(),
        };
        assert_eq!(e5.to_string(), "foo?");

        // 测试表达式的 `?` 运算
        let e6 = UnaryExpression {
            operator: Token::Unwrap,
            operand: Box::new(new_addition_expression(1, 2)),
            range: new_range(),
        };
        assert_eq!(e6.to_string(), "(1 + 2)?");
    }

    #[test]
    fn test_function_call_expression() {
        let e1 = FunctionCallExpression {
            callee: Box::new(Expression::Identifier(new_identifier("foo"))),
            arguments: vec![],
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "foo()");

        let e2 = FunctionCallExpression {
            callee: Box::new(Expression::Identifier(new_identifier("foo"))),
            arguments: vec![Argument {
                name: None,
                value: Box::new(Expression::Literal(new_literal_integer(1))),
                range: new_range(),
            }],
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "foo(1)");

        // 使用命名参数
        let e3 = FunctionCallExpression {
            callee: Box::new(Expression::Identifier(new_identifier("foo"))),
            arguments: vec![Argument {
                name: Some("width".to_string()),
                value: Box::new(Expression::Literal(new_literal_integer(2))),
                range: new_range(),
            }],
            range: new_range(),
        };
        assert_eq!(e3.to_string(), "foo(width=2)");

        // 表达式作为参数
        let e4 = FunctionCallExpression {
            callee: Box::new(Expression::Identifier(new_identifier("foo"))),
            arguments: vec![
                Argument {
                    name: Some("length".to_string()),
                    value: Box::new(new_addition_expression(1, 2)),
                    range: new_range(),
                },
                Argument {
                    name: Some("width".to_string()),
                    value: Box::new(Expression::Literal(new_literal_integer(3))),
                    range: new_range(),
                },
            ],
            range: new_range(),
        };
        assert_eq!(e4.to_string(), "foo(length=(1 + 2), width=3)");

        // callee 为表达式的情况
        let e5 = FunctionCallExpression {
            callee: Box::new(Expression::BinaryExpression(BinaryExpression {
                operator: Token::Combine,
                left: Box::new(Expression::Identifier(new_identifier("foo"))),
                right: Box::new(Expression::Identifier(new_identifier("bar"))),
                range: new_range(),
            })),
            arguments: vec![
                Argument {
                    name: None,
                    value: Box::new(Expression::Literal(new_literal_integer(10))),
                    range: new_range(),
                },
                Argument {
                    name: Some("name".to_string()),
                    value: Box::new(Expression::Literal(new_literal_integer(20))),
                    range: new_range(),
                },
            ],
            range: new_range(),
        };
        assert_eq!(e5.to_string(), "(foo & bar)(10, name=20)");
    }

    #[test]
    fn test_member_expression() {
        let e1 = MemberExpression {
            is_computed: false,
            object: Box::new(Expression::Identifier(new_identifier("foo"))),
            property: Box::new(Expression::Identifier(new_identifier("bar"))),
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "foo.bar");

        let e2 = MemberExpression {
            is_computed: true,
            object: Box::new(Expression::Identifier(new_identifier("foo"))),
            property: Box::new(Expression::Identifier(new_identifier("bar"))),
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "foo[bar]");

        // 索引为一个表达式
        let e3 = MemberExpression {
            is_computed: true,
            object: Box::new(Expression::Identifier(new_identifier("foo"))),
            property: Box::new(new_addition_expression(1, 2)),
            range: new_range(),
        };
        assert_eq!(e3.to_string(), "foo[(1 + 2)]");
    }

    #[test]
    fn test_slice_expression() {
        let e1 = SliceExpression {
            object: Box::new(Expression::Identifier(new_identifier("foo"))),
            interval: Interval {
                is_inclusive: false,
                from: Box::new(Expression::Literal(new_literal_integer(1))),
                to: Some(Box::new(Expression::Literal(new_literal_integer(5)))),
                range: new_range(),
            },
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "foo[1..5]");

        let e2 = SliceExpression {
            object: Box::new(Expression::Identifier(new_identifier("foo"))),
            interval: Interval {
                is_inclusive: true,
                from: Box::new(Expression::Literal(new_literal_integer(1))),
                to: Some(Box::new(Expression::Literal(new_literal_integer(5)))),
                range: new_range(),
            },
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "foo[1..=5]");

        let e3 = SliceExpression {
            object: Box::new(Expression::Identifier(new_identifier("foo"))),
            interval: Interval {
                is_inclusive: false,
                from: Box::new(Expression::Literal(new_literal_integer(1))),
                to: None,
                range: new_range(),
            },
            range: new_range(),
        };
        assert_eq!(e3.to_string(), "foo[1..]");
    }

    #[test]
    fn test_constructor_expression() {
        // 使用花括号方式实例化结构体的表达式
        let e1 = ConstructorExpression {
            object: new_identifier("Point"),
            value: Map {
                elements: vec![
                    MapEntry {
                        key: Box::new(Expression::Identifier(new_identifier("x"))),
                        value: Some(Box::new(Expression::Literal(new_literal_integer(123)))),
                        range: new_range(),
                    },
                    MapEntry {
                        key: Box::new(Expression::Identifier(new_identifier("y"))),
                        value: Some(Box::new(Expression::Literal(new_literal_integer(456)))),
                        range: new_range(),
                    },
                ],
                range: new_range(),
            },
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "Point {x: 123\ny: 456}");

        // 单纯使用 key 实例化
        let e2 = ConstructorExpression {
            object: new_identifier("User"),
            value: Map {
                elements: vec![
                    MapEntry {
                        key: Box::new(Expression::Identifier(new_identifier("id"))),
                        value: None,
                        range: new_range(),
                    },
                    MapEntry {
                        key: Box::new(Expression::Identifier(new_identifier("name"))),
                        value: None,
                        range: new_range(),
                    },
                ],
                range: new_range(),
            },
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "User {id\nname}");
    }

    // general expressions

    #[test]
    fn test_block_expression() {
        let e1 = BlockExpression {
            is_explicit: true,
            body: vec![
                Expression::Literal(new_literal_integer(10)),
                new_addition_expression(1, 2),
                Expression::Identifier(new_identifier("name")),
            ],
            range: new_range(),
        };

        assert_eq!(
            e1.to_string(),
            trim_left_margin(
                "do {10
                    (1 + 2)
                    name}"
            )
        );

        // 隠式 do 表达式
        let e2 = BlockExpression {
            is_explicit: false,
            body: vec![
                Expression::Literal(new_literal_integer(10)),
                Expression::Identifier(new_identifier("name")),
            ],
            range: new_range(),
        };

        assert_eq!(
            e2.to_string(),
            trim_left_margin(
                "{10
                    name}"
            )
        );

        // 空 do 表达式
        let e3 = BlockExpression {
            is_explicit: true,
            body: vec![],
            range: new_range(),
        };

        assert_eq!(e3.to_string(), trim_left_margin("do {}"));
    }

    #[test]
    fn test_join_expression() {
        let e1 = JoinExpression {
            body: vec![
                Expression::Literal(new_literal_integer(10)),
                new_addition_expression(1, 2),
                Expression::Identifier(new_identifier("name")),
            ],
            range: new_range(),
        };

        assert_eq!(
            e1.to_string(),
            trim_left_margin(
                "join {10
                    (1 + 2)
                    name}"
            )
        );
    }

    #[test]
    fn test_let_expression() {
        let e1 = LetExpression {
            object: Box::new(Expression::Identifier(new_identifier("foo"))),
            value: Box::new(Expression::Literal(new_literal_integer(123))),
            range: new_range(),
        };
        assert_eq!(e1.to_string(), "let foo = 123");

        // 右手边值为表达式
        let e2 = LetExpression {
            object: Box::new(Expression::Identifier(new_identifier("bar"))),
            value: Box::new(new_addition_expression(1, 2)),
            range: new_range(),
        };
        assert_eq!(e2.to_string(), "let bar = (1 + 2)");

        // 左手边值为元组
        let e3 = LetExpression {
            object: Box::new(Expression::Tuple(Tuple {
                elements: vec![
                    Expression::Identifier(new_identifier("id")),
                    Expression::Identifier(new_identifier("name")),
                ],
                range: new_range(),
            })),
            value: Box::new(Expression::Identifier(new_identifier("user"))),
            range: new_range(),
        };
        assert_eq!(e3.to_string(), "let (id, name,) = user");
    }

    fn test_if_expression() {
        // todo::
    }

    fn test_for_expression() {
        // todo::

        // 测试 next 语句
    }

    fn test_each_expression() {
        // todo::
    }

    fn test_branch_expression() {
        // todo::
    }

    fn test_match_expression() {
        // todo::
    }

    // statements

    fn test_function_declaration() {
        // todo::
    }

    fn test_empty_function_declaration() {
        // todo::
    }

    fn test_pattern_function_declarationzs() {
        // todo::
    }

    fn test_namespace_statement() {
        // todo::
    }

    fn test_use_statement() {
        // todo::
    }

    fn test_const_declaration() {
        // todo::
    }

    fn test_member_struct_declaration() {
        // todo::
    }

    fn test_tuple_struct_declaration() {
        // todo::
    }

    fn test_empty_struct_declaration() {
        // todo::
    }

    fn test_union_declaration() {
        // todo::
    }

    fn test_trait_declaration() {
        // todo::
    }

    fn test_impl_statement() {
        // todo::
    }

    fn test_alias_statement() {
        // todo::
    }

    fn test_expression() {
        // todo::
    }
}
