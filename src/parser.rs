/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use crate::{
    ast::{
        AnonymousFunction, AnonymousParameter, BinaryExpression, Bit, BlockExpression, Boolean,
        Char, Complex, ConstructorExpression, DataType, Ellipsis, Expression, Float, GeneralString,
        HashString, Identifier, Integer, Interval, List, Literal, Map, MapEntry, NamedOperator,
        Node, PrefixIdentifier, Program, Range, Statement, Tuple, UnaryExpression, WhichEntry,
    },
    error::Error,
    token::{self, Token, TokenDetail},
};

pub fn parse(source_token_details: &[TokenDetail]) -> Result<Node, Error> {
    let program = parse_program(source_token_details)?;
    Ok(Node::Program(program))
}

// Program
//  : StatementList
//  ;
//
// StatementList
//  : Statement
//  | StatementList NEW_LINE Statement
//  ;
fn parse_program(source_token_details: &[TokenDetail]) -> Result<Program, Error> {
    let mut token_details = source_token_details;
    let mut statements = Vec::<Statement>::new();

    loop {
        // 消除前导的空行
        let post_new_lines = skip_new_lines(token_details);

        if post_new_lines.first() == None {
            break;
        }

        let (statement, post_parse_statement) = parse_statement(post_new_lines)?;
        statements.push(statement);

        // 再解析剩余的 token，直到解析完所有 token 为止
        token_details = post_parse_statement;
    }

    Ok(Program {
        body: statements,
        range: new_range(),
    })
}

// Statement
//  : FunctionDeclaration
//  | EmptyFunctionDeclaration
//  | PatternFunctionDeclaration
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
fn parse_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    let first = &source_token_details[0];
    match first.token {
        Token::Function => parse_function_declaration(source_token_details),
        Token::Empty => parse_empty_function_declaration(source_token_details),
        Token::Pattern => parse_pattern_function_declaration(source_token_details),
        Token::Namespace => parse_namespace_statement(source_token_details),
        Token::Use => parse_use_statement(source_token_details),
        Token::Const => parse_const_statement(source_token_details),
        Token::Struct => parse_struct(source_token_details),
        Token::Union => parse_union(source_token_details),
        Token::Trait => parse_trait_declaration(source_token_details),
        Token::Impl => parse_impl_statement(source_token_details),
        Token::Alias => parse_alias_statement(source_token_details),
        _ => {
            // 表达式语句
            parse_expression_statement(source_token_details)
        }
    }
}

fn parse_function_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_empty_function_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_pattern_function_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_namespace_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_use_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_const_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_struct(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_union(source_token_details: &[TokenDetail]) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_trait_declaration(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_impl_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

fn parse_alias_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

// ExpressionStatement
//  : Expression NEW_LINE
//  | Expression EOF
//  ;
fn parse_expression_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    let (expression, rest) = parse_expression(source_token_details)?;

    // statement 以 Token::NewLine 或者 EOF 结束，消耗这个换行符（如果存在的话）
    consume_new_line_or_end_of_file(rest)
        .map(|post_rest| (Statement::Expression(expression), post_rest))
}

// Expression
//  : BlockExpression
//  | JoinExpression
//  | LetExpression
//
//  | IfExpression
//  | ForExpression
//  | NextExpression
//  | EachExpression
//  | BranchExpression
//  | MatchExpression
//  | SignExpression
//
//  | BinaryExpression
//  | UnaryExpression
//  | FunctionCallExpression
//  | MemberExpression
//  | SliceExpression
//  | ConstructorExpression
//
//  | AnonymousFunction
//  | Identifier
//  | PrefixIdentifier
//  | Tuple
//  | List
//  | Map
//  | Literal
//  ;

fn parse_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    if let Some(first) = source_token_details.first() {
        match first.token {
            Token::Do => parse_do_expression(source_token_details),
            Token::Join => parse_join_expression(source_token_details),
            Token::Let => parse_let_expression(source_token_details),
            Token::If => parse_if_expression(source_token_details),
            Token::For => parse_for_expression(source_token_details),
            Token::Next => parse_next_expression(source_token_details),
            Token::Each => parse_each_expression(source_token_details),
            Token::Branch => parse_branch_expression(source_token_details),
            Token::Match => parse_match_expression(source_token_details),
            Token::Sign => parse_sign_expression(source_token_details),
            _ => {
                // 二元运算表达式的开始
                parse_pipe_expression(source_token_details)
            }
        }
    } else {
        Err(Error::ParserError("expected expression".to_string()))
    }
}

// BlockExpression
//  : 'do' BlockExpression
//  ;
fn parse_do_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 do 表达式 `do {...}`，do 表达式是一个显式表达式块

    // 消除 do
    let post_consume_token_do = consume_token(&Token::Do, source_token_details)?;

    // 消除换行符
    // do 关键字后面允许换行
    let post_consume_new_lines = skip_new_lines(post_consume_token_do);

    let (expressions, post_parse_expression_block) =
        continue_parse_expression_block(post_consume_new_lines)?;

    Ok((
        Expression::BlockExpression(BlockExpression {
            is_explicit: true,
            body: expressions,
            range: new_range(),
        }),
        post_parse_expression_block,
    ))
}

// BlockExpression
//  : '{' ExpressionList '}'
//  ;
//
// ExpressionList
//  : Expression
//  | ExpressionList NEW_LINE Expression
//  ;
fn continue_parse_expression_block(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<Expression>, &[TokenDetail]), Error> {
    // 解析表达式块 `{...}`（也叫 `隠式 Do 表达式`）
    // 注意表达式块仅存在某些关键字后面，比如 `join`、`do` 等，而不能单独存在，
    // 当一对花括号单独存在时，会被解析为 Map。
    let mut token_details = source_token_details;

    token_details = consume_token(&Token::LeftBrace, token_details)?;

    let mut expressions: Vec<Expression> = vec![];

    loop {
        // 左花括号 '{' 后面允许换行
        // 表达式之间也是以换行分隔
        let post_consume_new_lines = skip_new_lines(token_details);

        // 解析表达式
        let (expression, post_parse_expression) = parse_expression(post_consume_new_lines)?;
        expressions.push(expression);

        token_details = post_parse_expression;

        if is_token_ignore_new_lines(&Token::RightBrace, post_parse_expression) {
            break;
        }
    }

    // 消除空行
    let post_consume_new_lines = skip_new_lines(token_details);
    let post_consume_token_right_brace = consume_token(&Token::RightBrace, post_consume_new_lines)?;

    Ok((expressions, post_consume_token_right_brace))
}

fn continue_parse_expression_block_or_single_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 `{...}` 或者 `...`
    // 在诸如 `if`、`then`、`else` 等关键字后面，即可以是单独一个表达式，
    // 也可以是一个表达式块。
    //
    // 解析优先级：
    // 1. 如果存在 `{...}`，则解析为隠式表达式块
    // 2. 否则解析为普通的表达式

    match source_token_details.first() {
        Some(first) => match first.token {
            Token::LeftBrace => {
                let (expressions, post_parse_expression_block) =
                    continue_parse_expression_block(source_token_details)?;

                Ok((
                    Expression::BlockExpression(BlockExpression {
                        is_explicit: false,
                        body: expressions,
                        range: new_range(),
                    }),
                    post_parse_expression_block,
                ))
            }
            _ => parse_expression(source_token_details),
        },
        None => Err(Error::ParserError(
            "expected an expression or an expression block".to_string(),
        )),
    }
}

fn parse_join_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // join {...}
    todo!()
}

fn parse_let_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // let left = right
    todo!()
}

fn parse_if_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // if ... then ... else ...
    todo!()
}

fn parse_for_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // for let ... = ... {... next}
    todo!()
}

fn parse_next_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // for let ... = ... {... next}
    todo!()
}

fn parse_each_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // each let ... in ... {...}
    todo!()
}

fn parse_branch_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // branch {...}
    todo!()
}

fn parse_match_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // match ... {...}
    todo!()
}

fn parse_sign_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // `sign (Int x, Int y) type Int`
    // `sign <T, E> (T x, E y) type T`
    // `sign (T a, String s) which {T: Int}`
    todo!()
}

fn continue_parse_generic_names(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<String>, &[TokenDetail]), Error> {
    // <A, B, C>
    // ^--- 当前位置
    todo!()
}

fn continue_parse_type_expression(
    source_token_details: &[TokenDetail],
) -> Result<(DataType, &[TokenDetail]), Error> {
    // type ...
    // ~~~~
    //    |-- 当前位置

    // 消除 `type` 关键字
    let post_type_token = consume_token(&Token::Type, source_token_details)?;
    // 消除空行
    let post_new_lines = skip_new_lines(post_type_token);

    let (data_type_expression, post_parse_data_type_expression) = parse_expression(post_new_lines)?;
    let data_type = convert_expression_to_data_type(data_type_expression)?;
    Ok((data_type, post_parse_data_type_expression))
}

fn continue_parse_which_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Vec<WhichEntry>, &[TokenDetail]), Error> {
    // which ...
    // which {...}
    // ~~~~~
    //     |-- 当前位置
    todo!()
}

fn continue_parse_where_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // which ...
    // ~~~~~
    //     |-- 当前位置
    todo!()
}

// 解析 `从左向右` 结合的二元运算的通用函数
//
// BinaryExpression
//  : NextExpression
//  | BinaryExpression OPERATOR NextExpression
//  ;
fn parse_binary_expression<'a>(
    operator_tokens: &[Token],
    next_parse_function: fn(&[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error>,
    source_token_details: &'a [TokenDetail],
) -> Result<(Expression, &'a [TokenDetail]), Error> {
    let mut token_details = source_token_details;

    let (mut left, post_parse_left_expression) = next_parse_function(token_details)?;
    token_details = post_parse_left_expression;

    loop {
        let next_token = match token_details.first() {
            Some(first) => &first.token,
            None => {
                break;
            }
        };

        let index = match operator_tokens.iter().position(|t| t == next_token) {
            Some(i) => i,
            None => {
                break;
            }
        };

        let operator_token = &operator_tokens[index];

        // 消除操作符
        let post_consume_token_operator = consume_token(operator_token, token_details)?;

        // 二元运算符后面允许换行
        let post_consume_new_lines = skip_new_lines(post_consume_token_operator);

        let (right, post_parse_right_expression) = next_parse_function(post_consume_new_lines)?;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
        token_details = post_parse_right_expression;
    }

    Ok((left, token_details))
}

// 解析 `从右向左` 结合的二元运算的通用函数
//
// BinaryExpression
//  : NextExpression
//  | NextExpression OPERATOR Expression
//  ;
fn parse_right_2_left_binary_expression<'a>(
    operator_token: &Token,
    next_parse_function: fn(&[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error>,
    source_token_details: &'a [TokenDetail],
) -> Result<(Expression, &'a [TokenDetail]), Error> {
    let mut token_details = source_token_details;

    let (mut left, post_parse_left_expression) = next_parse_function(token_details)?;
    token_details = post_parse_left_expression;

    if is_token(operator_token, token_details) {
        // 消除操作符
        let post_consume_token_operator = consume_token(operator_token, token_details)?;

        // 二元运算符后面允许换行
        let pose_consume_new_lines = skip_new_lines(post_consume_token_operator);

        let (right, post_parse_right_expression) = parse_expression(pose_consume_new_lines)?;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
        token_details = post_parse_right_expression;
    }

    Ok((left, token_details))
}

fn parse_pipe_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left | right
    parse_binary_expression(
        &vec![Token::Pipe],
        parse_logic_or_expression,
        source_token_details,
    )
}

fn parse_logic_or_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left || right
    parse_binary_expression(
        &vec![Token::LogicOr],
        parse_logic_and_expression,
        source_token_details,
    )
}

fn parse_logic_and_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left && right
    parse_binary_expression(
        &vec![Token::LogicAnd],
        parse_equality_expression,
        source_token_details,
    )
}

fn parse_equality_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left == right, left != right
    parse_binary_expression(
        &vec![Token::Equal, Token::NotEqual],
        parse_relational_expression,
        source_token_details,
    )
}

fn parse_relational_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left > right, left >= right, left < right, left <= right
    parse_binary_expression(
        &vec![
            Token::GreaterThan,
            Token::GreaterThanOrEqual,
            Token::LessThan,
            Token::LessThanOrEqual,
        ],
        parse_named_operator_expression,
        source_token_details,
    )
}

fn parse_named_operator_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left :bitOr: right
    //
    // 注：
    // 命名操作符无法使用通用的二元运算解析函数 parse_binary_expression
    let mut token_details = source_token_details;

    let (mut left, post_parse_left_expression) = parse_concat_expression(token_details)?;
    token_details = post_parse_left_expression;

    if let Some(TokenDetail {
        token: named_operator_token @ Token::NamedOperator(_),
        ..
    }) = token_details.first()
    {
        // 消除操作符
        let post_consume_token_operator = consume_token(named_operator_token, token_details)?;

        // 二元运算符后面允许换行
        let pose_consume_new_lines = skip_new_lines(post_consume_token_operator);

        let (right, post_parse_right_expression) = parse_concat_expression(pose_consume_new_lines)?;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: named_operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
        token_details = post_parse_right_expression;
    }

    Ok((left, token_details))
}

fn parse_concat_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left ++ right
    parse_binary_expression(
        &vec![Token::Concat],
        parse_additive_expression,
        source_token_details,
    )
}

fn parse_additive_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left + right, left - right
    parse_binary_expression(
        &vec![Token::Plus, Token::Minus],
        parse_multiplicative_expression,
        source_token_details,
    )
}

fn parse_multiplicative_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left * right, left / right
    parse_binary_expression(
        &vec![Token::Asterisk, Token::Slash],
        parse_optional_or_expression,
        source_token_details,
    )
}

fn parse_optional_or_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left ?? right
    parse_binary_expression(
        &vec![Token::OptionalOr],
        parse_optional_and_expression,
        source_token_details,
    )
}

fn parse_optional_and_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left >> right
    parse_binary_expression(
        &vec![Token::OptionalAnd],
        parse_combine_expression,
        source_token_details,
    )
}

fn parse_combine_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left & right
    // 结合方向：从右向左
    parse_right_2_left_binary_expression(
        &Token::Combine,
        parse_cast_expression,
        source_token_details,
    )
}

fn parse_cast_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 object^
    let (left, post_parse_expression) = parse_negative_expression(source_token_details)?;

    if is_token(&Token::Cast, post_parse_expression) {
        let post_consume_token_operator = consume_token(&Token::Cast, post_parse_expression)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Cast,
                operand: Box::new(left),
                range: new_range(),
            }),
            post_consume_token_operator,
        ))
    } else {
        Ok((left, post_parse_expression))
    }
}

fn parse_negative_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 -object
    if is_token(&Token::Minus, source_token_details) {
        let post_consume_token_operator = consume_token(&Token::Cast, source_token_details)?;
        let (left, post_parse_expression) = parse_unwrap_expression(post_consume_token_operator)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Minus,
                operand: Box::new(left),
                range: new_range(),
            }),
            post_parse_expression,
        ))
    } else {
        parse_unwrap_expression(source_token_details)
    }
}

fn parse_unwrap_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 object?
    let (left, post_parse_expression) = parse_function_call_expression(source_token_details)?;

    if is_token(&Token::Unwrap, post_parse_expression) {
        let post_consume_token_operator = consume_token(&Token::Unwrap, post_parse_expression)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Unwrap,
                operand: Box::new(left),
                range: new_range(),
            }),
            post_consume_token_operator,
        ))
    } else {
        Ok((left, post_parse_expression))
    }
}

fn parse_function_call_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // foo()
    // todo::
    parse_member_or_slice_expression(source_token_details)
}

fn parse_member_or_slice_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // object.property, object["foo"]
    // todo::
    parse_constructor_expression(source_token_details)
}

fn parse_constructor_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // object {name: vale, ...}

    let (object, post_parse_primary_expression) = parse_primary_expression(source_token_details)?;
    if is_token(&Token::LeftBrace, post_parse_primary_expression) {
        let (initializer, post_continue_parse_map) =
            continue_parse_map(post_parse_primary_expression)?;

        if let Expression::Identifier(identifier) = object {
            let exp = Expression::ConstructorExpression(ConstructorExpression {
                object: identifier,
                value: initializer,
                range: new_range(),
            });

            Ok((exp, post_continue_parse_map))
        } else {
            Err(Error::ParserError("invalid constructor object".to_string()))
        }
    } else {
        Ok((object, post_parse_primary_expression))
    }
}

// PrimaryExpression
//  : Fn
//  | Tuple/Parenthesized
//  | List
//  | Map
//  | Identifier
//  | Literal
//  ;
fn parse_primary_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    match source_token_details.first() {
        Some(first) => match first.token {
            Token::Fn => parse_anonymous_function(source_token_details),
            Token::LeftParen => parse_tuple_or_parenthesized(source_token_details),
            Token::LeftBracket => parse_list(source_token_details),
            Token::LeftBrace => parse_map(source_token_details),
            Token::Exclamation => parse_prefix_identifier(source_token_details), // 函数的前置调用
            Token::Identifier(_) => parse_identifier(source_token_details),
            _ => {
                let (literal, post_parse_literal) = parse_literal(source_token_details)?;
                Ok((Expression::Literal(literal), post_parse_literal))
            }
        },
        None => Err(Error::ParserError(
            "expected primary expression".to_string(),
        )),
    }
}

fn parse_anonymous_function(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 匿名函数
    // fn (Int a, Int b) type Int = ...
    // fn (Int a, Int b) = ...
    // fn (a, b) = ...
    // fn x = ...            // 当参数只有一个，且省略数据类型时，可以省略参数的括号
    // fn x {...}            // 匿名函数的主体也有可能是 `隠式 do 表达式`

    let mut token_details = source_token_details;

    let mut parameters: Vec<AnonymousParameter> = vec![];
    let mut return_data_type: Option<DataType> = None;
    let mut which_entries: Vec<WhichEntry> = vec![];
    let mut where_exp: Option<Box<Expression>> = None;

    let mut is_expected_end = false;

    token_details = consume_token(&Token::Fn, token_details)?;
    token_details = skip_new_lines(token_details); // 关键字后允许换行

    let post_parameters = match token_details.split_first() {
        Some((maybe_left_paren, post_left_paren)) if maybe_left_paren.token == Token::LeftParen => {
            // 参数列表有括号包围
            token_details = post_left_paren;

            // 解析参数列表
            loop {
                token_details = match token_details.first() {
                    Some(first) => {
                        if let TokenDetail {
                            token: Token::RightParen,
                            ..
                        } = first
                        {
                            // 找到了结束符号————右括号，退出循环
                            break;
                        } else {
                            if is_expected_end {
                                // 当前的状态是一心寻找结束符号 ———— 右括号
                                return Err(Error::ParserError(
                                    "expected the right paren symbol \")\"".to_string(),
                                ));
                            } else {
                                // 先尝试寻找参数的数据类型
                                let (part_one, post_part_one) = parse_expression(token_details)?;

                                let post_one_parameter = match post_part_one.split_first() {
                                    Some((maybe_comma_or_right_paren, _))
                                        if maybe_comma_or_right_paren.token == Token::Comma
                                            || maybe_comma_or_right_paren.token
                                                == Token::RightParen =>
                                    {
                                        // 当前参数无数据类型
                                        if let Expression::Identifier(Identifier { name, .. }) =
                                            part_one
                                        {
                                            parameters.push(AnonymousParameter {
                                                data_type: None,
                                                name: name,
                                                range: new_range(),
                                            });
                                            post_part_one
                                        } else {
                                            return Err(Error::ParserError(
                                                "invalid anonymous function parameter name"
                                                    .to_string(),
                                            ));
                                        }
                                    }
                                    Some((
                                        TokenDetail {
                                            token: Token::Identifier(name),
                                            ..
                                        },
                                        post_part_two,
                                    )) => {
                                        // 当前参数有数据类型
                                        let data_type = convert_expression_to_data_type(part_one)?;
                                        parameters.push(AnonymousParameter {
                                            data_type: Some(data_type),
                                            name: name.clone(),
                                            range: new_range(),
                                        });
                                        post_part_two
                                    }
                                    _ => {
                                        return Err(Error::ParserError(
                                            "incomplete anonymous function parameter".to_string(),
                                        ));
                                    }
                                };

                                // 消除逗号
                                let post_consume_comma =
                                    if is_token(&Token::Comma, post_one_parameter) {
                                        consume_token(&Token::Comma, post_one_parameter)?
                                    } else {
                                        // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                                        // 后面只能允许列表结束
                                        is_expected_end = true;
                                        post_one_parameter
                                    };

                                // 消除空行
                                let post_consume_new_lines = skip_new_lines(post_consume_comma);
                                post_consume_new_lines
                            }
                        }
                    }
                    None => {
                        return Err(Error::ParserError(
                            "expected the right paren symbol \")\"".to_string(),
                        ));
                    }
                }
            }

            // 消除右括号
            consume_token(&Token::RightParen, token_details)?
        }
        Some((
            TokenDetail {
                token: Token::Identifier(name),
                ..
            },
            post_left_paren,
        )) => {
            // 参数列表只有一个参数，且无括号包围
            parameters.push(AnonymousParameter {
                data_type: None,
                name: name.clone(),
                range: new_range(),
            });
            post_left_paren
        }
        _ => {
            return Err(Error::ParserError(
                "expected anonymous function parameter".to_string(),
            ));
        }
    };

    token_details = post_parameters;

    loop {
        // 消除参数列表后面（首次运行 loop 主体时），以及各个从属表达式后面的空行
        let post_new_lines = skip_new_lines(token_details);

        // 尝试解析 type, which, where 等从属表达式
        token_details = match post_new_lines.first() {
            Some(t) if t.token == Token::Type => {
                let (data_type, post_parse_data_type_expression) =
                    continue_parse_type_expression(post_new_lines)?;

                return_data_type = Some(data_type);
                post_parse_data_type_expression
            }
            Some(t) if t.token == Token::Which => {
                let (entries, post_parse_which_expression) =
                    continue_parse_which_expression(post_new_lines)?;

                which_entries = entries;
                post_parse_which_expression
            }
            Some(t) if t.token == Token::Where => {
                let (exp, post_parse_where_expression) =
                    continue_parse_where_expression(post_new_lines)?;

                where_exp = Some(Box::new(exp));
                post_parse_where_expression
            }
            _ => {
                break;
            }
        }
    }

    // 消除赋值符号（如果存在的话）
    let post_assignment = if is_token(&Token::Assign, token_details) {
        let post_assignment_token = consume_token(&Token::Assign, token_details)?;
        // 消除空行
        skip_new_lines(post_assignment_token)
    } else {
        token_details
    };

    // 解析函数主体
    let (body, post_body) = continue_parse_expression_block_or_single_expression(post_assignment)?;

    // 构造匿名函数对象
    let anonymous_function = AnonymousFunction {
        parameters: parameters,
        return_data_type: return_data_type,
        which_entries: which_entries,
        where_exp: where_exp,
        body: Box::new(body),
        range: new_range(),
    };

    Ok((Expression::AnonymousFunction(anonymous_function), post_body))
}

fn convert_expression_to_data_type(exp: Expression) -> Result<DataType, Error> {
    match exp {
        Expression::Identifier(identifier) => Ok(DataType::Identifier(identifier)),
        Expression::Sign(sign) => Ok(DataType::Sign(sign)),
        Expression::Tuple(tuple) => Ok(DataType::Tuple(tuple)),
        _ => Err(Error::ParserError(
            "invalid anonymous function parameter data type".to_string(),
        )),
    }
}

fn parse_list(source_token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    // list
    //
    // e.g.
    // [123, 345, 567,]
    // [123, 345, 567]   // 末尾逗号可省略
    //
    // [
    //    123,  // 换行时，项目之间的逗号不可以省略
    //    456,
    //    678
    // ]
    //
    // [1..10]
    // [1..=9]
    // [1,3..10]

    let mut token_details = source_token_details;

    let mut expressions: Vec<Expression> = vec![];
    let mut is_expected_end = false; // 标记当前是否处于一心找列表结束的状态

    token_details = consume_token(&Token::LeftBracket, token_details)?; // 消除左中括号（方括号）
    token_details = skip_new_lines(token_details); // 左中括号（方括号）后面允许换行

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if let TokenDetail {
                    token: Token::RightBracket,
                    ..
                } = first
                {
                    // 找到了结束符号————右中括号（方括号），退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 ———— 右中括号（方括号）
                        return Err(Error::ParserError(
                            "expected the right bracket symbol \"]\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `省略符表达式`
                        if let TokenDetail {
                            token: Token::Ellipsis,
                            ..
                        } = first
                        {
                            // 当前是 `省略符表达式`
                            let (ellipsis, post_parse_ellipsis) =
                                continue_parse_ellipsis(token_details)?;
                            expressions.push(Expression::Ellipsis(ellipsis));
                            is_expected_end = true; // 设置标记，`省略符表达式` 后面只能允许列表结束

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_parse_ellipsis)
                            {
                                consume_token(&Token::Comma, post_parse_ellipsis)?
                            } else {
                                post_parse_ellipsis
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 当前是普通表达式或者 `范围表达式`
                            let (expression, post_parse_expression) =
                                parse_expression(token_details)?;

                            let post_check_interval =
                                if is_token(&Token::Interval, post_parse_expression)
                                    || is_token(&Token::IntervalInclusive, post_parse_expression)
                                {
                                    // 当前是 `范围表达式`
                                    let (
                                        is_inclusive,
                                        optional_to_expression,
                                        post_continue_parse_interval,
                                    ) = continue_parse_interval(post_parse_expression)?;

                                    let interval_expression = Expression::Interval(Interval {
                                        is_inclusive,
                                        from: Box::new(expression),
                                        to: match optional_to_expression {
                                            Some(end_expression) => Some(Box::new(end_expression)),
                                            None => None,
                                        },
                                        range: new_range(),
                                    });

                                    is_expected_end = true; // 设置标记，`范围表达式` 后面只能允许列表结束

                                    expressions.push(interval_expression);
                                    post_continue_parse_interval
                                } else {
                                    // 当前是普通表达式
                                    expressions.push(expression);
                                    post_parse_expression
                                };

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_check_interval)
                            {
                                consume_token(&Token::Comma, post_check_interval)?
                            } else {
                                // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                                // 后面只能允许列表结束
                                is_expected_end = true;
                                post_check_interval
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        }
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right bracket symbol \")\"".to_string(),
                ))
            }
        }
    }

    // 消除右括号
    token_details = consume_token(&Token::RightBracket, token_details)?;

    Ok((
        Expression::List(List {
            elements: expressions,
            range: new_range(),
        }),
        token_details,
    ))
}

fn parse_tuple_or_parenthesized(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // tuple or parenthesized
    //
    // e.g.
    //
    // tuple:
    //
    // ()
    // (one,)              // 单独一个元素的末尾逗号不可省略
    // (one, two, )        // 末尾逗号可以省略
    // (one, two, ...)     //
    // (one, two, ...rest) //
    //
    // (
    //    123,  // 换行时，项目之间的逗号不可以省略
    //    456,
    //    678
    // )
    //
    // parenthesized:
    //
    // (expression)

    let mut token_details = source_token_details;

    let mut expressions: Vec<Expression> = vec![];
    let mut is_tuple = false; // 标记当前是否元组（而不是表达式括号运算）
    let mut is_expected_end = false; // 标记当前是否处于一心找元组结束的状态

    token_details = consume_token(&Token::LeftParen, token_details)?; // 消除左括号
    token_details = skip_new_lines(token_details); // 左括号后面允许换行

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if let TokenDetail {
                    token: Token::RightParen,
                    ..
                } = first
                {
                    // 找到了结束符号————右括号，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 ———— 右括号
                        return Err(Error::ParserError(
                            "expected the right paren symbol \")\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `省略符表达式`
                        if let TokenDetail {
                            token: Token::Ellipsis,
                            ..
                        } = first
                        {
                            // 当前是 `省略符表达式`
                            let (ellipsis, post_parse_ellipsis) =
                                continue_parse_ellipsis(token_details)?;
                            expressions.push(Expression::Ellipsis(ellipsis));
                            is_expected_end = true; // 设置标记，`省略符表达式` 后面只能允许列表结束

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_parse_ellipsis)
                            {
                                consume_token(&Token::Comma, post_parse_ellipsis)?
                            } else {
                                post_parse_ellipsis
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 当前是普通表达式
                            let (expression, post_parse_expression) =
                                parse_expression(token_details)?;
                            expressions.push(expression);

                            // 消除逗号
                            let post_consume_comma =
                                if is_token(&Token::Comma, post_parse_expression) {
                                    // 检测到逗号，设置标记，表明当前表达式是元组而非括号表达式
                                    is_tuple = true;
                                    consume_token(&Token::Comma, post_parse_expression)?
                                } else {
                                    // 设置标记，表示如果项目后面没有逗号，则表示当前已经是最后一项
                                    // 后面只能允许列表结束
                                    is_expected_end = true;
                                    post_parse_expression
                                };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        }
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right paren symbol \")\"".to_string(),
                ))
            }
        }
    }

    // 消除右括号
    token_details = consume_token(&Token::RightParen, token_details)?;

    if expressions.len() == 0 {
        // 空元组
        Ok((
            Expression::Tuple(Tuple {
                elements: vec![],
                range: new_range(),
            }),
            token_details,
        ))
    } else {
        if is_tuple {
            // 元组
            Ok((
                Expression::Tuple(Tuple {
                    elements: expressions,
                    range: new_range(),
                }),
                token_details,
            ))
        } else {
            // 普通的括号表达式
            Ok((expressions[0].clone(), token_details))
        }
    }
}

fn continue_parse_ellipsis(
    source_token_details: &[TokenDetail],
) -> Result<(Ellipsis, &[TokenDetail]), Error> {
    // ...
    // ..._
    // ...abc
    // ^  ^--- identifier
    // |------ ellipsis，当前处于这个 token

    // 消除省略号 `...`
    let post_consume_token_ellipsis = consume_token(&Token::Ellipsis, source_token_details)?;

    if let Some((
        TokenDetail {
            token: Token::Identifier(name),
            ..
        },
        post_consume_token_identifier,
    )) = post_consume_token_ellipsis.split_first()
    {
        // 省略号 `...` 后面有标识符
        Ok((
            Ellipsis {
                name: Some(name.clone()),
                range: new_range(),
            },
            post_consume_token_identifier,
        ))
    } else {
        // 省略号 `...` 后面无标识符
        Ok((
            Ellipsis {
                name: None,
                range: new_range(),
            },
            post_consume_token_ellipsis,
        ))
    }
}

// 解析 `范围表达式`
// 返回 (`to` 是否闭区间, `to` 表达式, 剩余的 token)
fn continue_parse_interval(
    source_token_details: &[TokenDetail],
) -> Result<(bool, Option<Expression>, &[TokenDetail]), Error> {
    // exp1..=
    // exp1..=exp2
    // exp1..
    // exp1..exp2
    // ^   ^ ^--- expression (可选的)
    // |   |----- interval 当前处于这个 token
    // |--------- expression

    let is_inclusive = is_token(&Token::IntervalInclusive, source_token_details);
    let operator_token = if is_inclusive {
        Token::IntervalInclusive
    } else {
        Token::Interval
    };

    // 消除范围符号 ".." 或者 "..="
    let post_consume_token_interval = consume_token(&operator_token, source_token_details)?;

    // 范围符号 ".."  或者 "..=" 后面允许换行
    let post_new_lines = skip_new_lines(post_consume_token_interval);

    match post_new_lines.first() {
        Some(TokenDetail { token, .. })
            if (*token == Token::Comma || *token == Token::RightBracket) =>
        {
            // 遇到了逗号或者右中括号（方括号）
            if is_inclusive {
                // 对于闭区间的范围表达式，`to` 部分是不能省略的。
                Err(Error::ParserError(
                    "expected inclusive range end".to_string(),
                ))
            } else {
                // 当前范围表达式缺省了 `to` 部分。
                Ok((is_inclusive, None, post_new_lines))
            }
        }
        _ => {
            // 解析 `to` 部分表达式
            let (to_expression, post_parse_to_expression) = parse_expression(post_new_lines)?;
            Ok((is_inclusive, Some(to_expression), post_parse_to_expression))
        }
    }
}

fn parse_map(source_token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    let (map, post_continue_parse_map) = continue_parse_map(source_token_details)?;
    Ok((Expression::Map(map), post_continue_parse_map))
}

fn continue_parse_map(
    source_token_details: &[TokenDetail],
) -> Result<(Map, &[TokenDetail]), Error> {
    // map
    //
    // e.g.
    // {name: value, name: value} // 项目之间使用逗号分隔
    // {id, name, ...rest}        // 末尾逗号可以省略
    // {
    //    id: value,
    //    name: value    // 换行时，项目之间的逗号 **可以** 省略
    //    checkd: value  // 这种格式对于同样使用花括号作为主体的 map/branch/match 三种表达式都保持一致
    // }

    let mut token_details = source_token_details;
    let mut entries: Vec<MapEntry> = vec![];
    let mut is_expected_end = false; // 标记当前是否处于一心找映射表结束的状态

    token_details = consume_token(&Token::LeftBrace, token_details)?; // 消除左花括号
    token_details = skip_new_lines(token_details); // 左花括号后面允许换行

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if let TokenDetail {
                    token: Token::RightBrace,
                    ..
                } = first
                {
                    // 找到了结束符号————右花括号，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前的状态是一心寻找结束符号 ———— 右花括号
                        return Err(Error::ParserError(
                            "expected the right brace symbol \"}\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `省略符表达式`
                        if let TokenDetail {
                            token: Token::Ellipsis,
                            ..
                        } = first
                        {
                            // 当前是 `省略符表达式`
                            let (ellipsis, post_parse_ellipsis) =
                                continue_parse_ellipsis(token_details)?;

                            // `省略表达式` 以 `key` 添加到项目里
                            entries.push(MapEntry {
                                key: Box::new(Expression::Ellipsis(ellipsis)),
                                value: None,
                                range: new_range(),
                            });
                            is_expected_end = true; // 设置标记，`省略符表达式` 后面只能允许列表结束

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_parse_ellipsis)
                            {
                                consume_token(&Token::Comma, post_parse_ellipsis)?
                            } else {
                                post_parse_ellipsis
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 当前是 `key: value` 表达式
                            // 注意其中的 `value` 部分是可选的。

                            let (expression, post_parse_key_expression) =
                                parse_expression(token_details)?;

                            let post_entry = if is_token(&Token::Colon, post_parse_key_expression) {
                                // 当前存在 `value` 部分

                                // 消除冒号
                                let post_consume_colon =
                                    consume_token(&Token::Colon, post_parse_key_expression)?;

                                // 消除空行
                                let post_consume_new_lines_after_colon =
                                    skip_new_lines(post_consume_colon);

                                let (value_expression, post_parse_value_expression) =
                                    parse_expression(post_consume_new_lines_after_colon)?;

                                // 构造 MapEntry
                                let entry = MapEntry {
                                    key: Box::new(expression),
                                    value: Some(Box::new(value_expression)),
                                    range: new_range(),
                                };

                                entries.push(entry);
                                post_parse_value_expression
                            } else {
                                // 当前不存在 `value` 部分

                                // 构造 MapEntry
                                let entry = MapEntry {
                                    key: Box::new(expression),
                                    value: None,
                                    range: new_range(),
                                };

                                entries.push(entry);
                                post_parse_key_expression
                            };

                            // 如果接下来是：
                            // - 逗号
                            // - 逗号+空行
                            // - 空行
                            //
                            // 表明还有下一项，否则表示后面没有更多项目

                            let post_consume_comma = match post_entry.split_first() {
                                Some((first, rest)) if first.token == Token::Comma => {
                                    // 消除逗号
                                    rest
                                }
                                Some((first, _)) if first.token == Token::NewLine => {
                                    // 等接下来的代码来统一来消除空行
                                    post_entry
                                }
                                _ => {
                                    // 没有下一项了，标记映射表的已经到达末尾
                                    is_expected_end = true;
                                    post_entry
                                }
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        }
                    }
                }
            }
            None => {
                return Err(Error::ParserError(
                    "expected the right brace symbol \"}\"".to_string(),
                ));
            }
        }
    }

    // 消除右花括号
    token_details = consume_token(&Token::RightBrace, token_details)?;

    Ok((
        Map {
            elements: entries,
            range: new_range(),
        },
        token_details,
    ))
}

fn parse_prefix_identifier(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // prefix identifier
    let post_consume_token_exclamation = consume_token(&Token::Exclamation, source_token_details)?;

    let (identifier, post_continue_parse_identifier) =
        continue_parse_identifier(post_consume_token_exclamation)?;

    Ok((
        Expression::PrefixIdentifier(PrefixIdentifier {
            identifier: identifier,
            range: new_range(),
        }),
        post_continue_parse_identifier,
    ))
}

fn parse_identifier(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // identifier
    //
    // One::Two::Three::Name
    let (identifier, post_continue_parse_identifier) =
        continue_parse_identifier(source_token_details)?;

    Ok((
        Expression::Identifier(identifier),
        post_continue_parse_identifier,
    ))
}

fn continue_parse_identifier(
    source_token_details: &[TokenDetail],
) -> Result<(Identifier, &[TokenDetail]), Error> {
    // identifier
    //
    // e.g.
    // One::Two::Three::Name
    let mut token_details = source_token_details;
    let mut names: Vec<String> = vec![];

    if let Some((
        TokenDetail {
            token: Token::Identifier(name),
            ..
        },
        rest,
    )) = token_details.split_first()
    {
        // 获取第一个 identifier
        names.push(name.clone());
        token_details = rest;

        // 获取其余的 identifier
        loop {
            token_details = match token_details.split_first() {
                Some((first, post_token_separator)) if first.token == Token::Separator => {
                    // 检测到 namespace path 分隔符 `::`
                    if let Some((
                        TokenDetail {
                            token: Token::Identifier(name),
                            ..
                        },
                        post_token_identifier,
                    )) = post_token_separator.split_first()
                    {
                        // 检测到一个 identifier
                        names.push(name.clone());
                        post_token_identifier
                    } else {
                        // 在 namespace path 分隔符 `::` 后面必须是一个 identifier
                        return Err(Error::ParserError("expected identifier".to_string()));
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }

    if names.len() == 0 {
        Err(Error::ParserError("expected identifier".to_string()))
    } else {
        let len = names.len();
        Ok((
            Identifier {
                dirs: names[..len - 1].iter().map(|n| n.clone()).collect(),
                name: names[len - 1].clone(),
                generic_names: vec![],
                range: new_range(),
            },
            token_details,
        ))
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

fn parse_literal(source_token_details: &[TokenDetail]) -> Result<(Literal, &[TokenDetail]), Error> {
    match source_token_details.split_first() {
        Some((first, rest)) => match &first.token {
            Token::Integer(v) => match continue_parse_imaginary(rest) {
                // 整数或复数
                Ok((f, post_rest)) => Ok((
                    Literal::Complex(Complex {
                        real: *v as f64,
                        imaginary: f,
                        range: new_range(),
                    }),
                    post_rest,
                )),
                _ => Ok((
                    Literal::Integer(Integer {
                        value: *v,
                        range: new_range(),
                    }),
                    rest,
                )),
            },
            Token::Float(v) => match continue_parse_imaginary(rest) {
                // 浮点数或复数
                Ok((f, post_rest)) => Ok((
                    Literal::Complex(Complex {
                        real: *v,
                        imaginary: f,
                        range: new_range(),
                    }),
                    post_rest,
                )),
                _ => Ok((
                    Literal::Float(Float {
                        value: *v,
                        range: new_range(),
                    }),
                    rest,
                )),
            },
            Token::Imaginary(v) => {
                // 只有单独虚部的复数
                Ok((
                    Literal::Complex(Complex {
                        real: 0f64,
                        imaginary: *v,
                        range: new_range(),
                    }),
                    rest,
                ))
            }
            Token::Bit(width, bytes) => Ok((
                Literal::Bit(Bit {
                    width: *width,
                    bytes: bytes.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            Token::Boolean(v) => Ok((
                Literal::Boolean(Boolean {
                    value: *v,
                    range: new_range(),
                }),
                rest,
            )),
            Token::Char(v) => Ok((
                Literal::Char(Char {
                    value: *v,
                    range: new_range(),
                }),
                rest,
            )),
            Token::GeneralString(v) => Ok((
                Literal::GeneralString(GeneralString {
                    value: v.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            Token::TemplateString(v) => {
                // todo::
                // 这里需要重新 tokenize 模板字符串里面的占位符表达式，
                // 然后重新解析这些表达式
                todo!()
            }
            Token::HashString(v) => Ok((
                Literal::HashString(HashString {
                    value: v.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            Token::NamedOperator(v) => Ok((
                Literal::NamedOperator(NamedOperator {
                    value: v.clone(),
                    range: new_range(),
                }),
                rest,
            )),
            _ => Err(Error::ParserError("unexpected literal".to_string())),
        },
        None => Err(Error::ParserError("expected literal".to_string())),
    }
}

// 尝试解析复数，如果成功则返回虚数及剩余的 token，
// 如果不成功则返回空元
fn continue_parse_imaginary(
    source_token_details: &[TokenDetail],
) -> Result<(f64, &[TokenDetail]), ()> {
    match source_token_details.split_first() {
        Some((first, rest)) if first.token == Token::Plus => match rest.split_first() {
            Some((
                TokenDetail {
                    token: Token::Imaginary(f),
                    ..
                },
                post_rest,
            )) => Ok((*f, post_rest)),
            _ => {
                // 当前表达式并非复数（但不是错误）
                Err(())
            }
        },
        _ => {
            // 当前表达式并非复数（但不是错误）
            Err(())
        }
    }
}

// 跳过空白的行，在 lexer 里产生的 Token 序列当中，有可能存在多行连续的空行，
// 在解析一个statement 之前，或者 expression 之间，需要消除这些空白的前导空行
fn skip_new_lines(source_token_details: &[TokenDetail]) -> &[TokenDetail] {
    let mut token_details = source_token_details;

    loop {
        token_details = match token_details.split_first() {
            Some((first, rest)) if first.token == Token::NewLine => rest,
            _ => {
                break;
            }
        }
    }

    token_details
}

// fn skip_new_lines_and_consume_token<'a>(
//     expected: &Token,
//     source_token_details: &'a [TokenDetail],
// ) -> Result<&'a [TokenDetail], Error> {
//     let token_details = skip_new_lines(source_token_details);
//     consume_token(expected, token_details)
// }

fn is_token(expected: &Token, source_token_details: &[TokenDetail]) -> bool {
    match source_token_details.first() {
        Some(first) if &first.token == expected => true,
        _ => false,
    }
}

fn is_token_ignore_new_lines(expected: &Token, source_token_details: &[TokenDetail]) -> bool {
    let token_details = skip_new_lines(source_token_details);
    is_token(expected, token_details)
}

fn consume_token<'a>(
    expected: &Token,
    source_token_details: &'a [TokenDetail],
) -> Result<&'a [TokenDetail], Error> {
    match source_token_details.split_first() {
        Some((first, rest)) if &first.token == expected => Ok(rest),
        _ => Err(Error::ParserError(format!(
            "expected the specified symbol \"{}\"",
            expected
        ))),
    }
}

// fn consume_token_if_exists<'a>(
//     expected: &Token,
//     source_token_details: &'a [TokenDetail],
// ) -> &'a [TokenDetail] {
//     match source_token_details.split_first() {
//         Some((first, rest)) if &first.token == expected => rest,
//         _ => source_token_details,
//     }
// }

fn consume_new_line_or_end_of_file(
    source_token_details: &[TokenDetail],
) -> Result<&[TokenDetail], Error> {
    match source_token_details.split_first() {
        Some((first, rest)) => {
            if first.token == Token::NewLine {
                Ok(rest)
            } else {
                Err(Error::ParserError(
                    "expected the new-line symbol".to_string(),
                ))
            }
        }
        None => Ok(source_token_details),
    }
}

fn new_range() -> Range {
    // todo::
    // 各成员的值应该有参数传入
    Range {
        file_id: 0,
        start: 0,
        end: 0,
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{
            BinaryExpression, BlockExpression, Complex, Ellipsis, Expression, Float, Identifier,
            Integer, Interval, List, Literal, Node, PrefixIdentifier, Program, Statement, Tuple,
        },
        error::Error,
        lexer,
        parser::new_range,
        token::Token,
    };

    use super::parse;

    // 辅助函数

    fn parse_from_string(text: &str) -> Result<Node, Error> {
        let token_details = lexer::tokenize(text)?;
        parse(&token_details)
    }

    fn trim_left_margin(s: &str) -> String {
        s.split("\n")
            .map(|s| s.trim_start().to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    // literal

    #[test]
    fn test_integer_literal() {
        let n1 = parse_from_string("123").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Literal(
                    Literal::Integer(Integer {
                        value: 123,
                        range: new_range()
                    })
                ))],
                range: new_range()
            })
        );

        assert_eq!(n1.to_string(), "123\n"); // Statement 以符号 '\n' 结尾
    }

    #[test]
    fn test_float_literal() {
        let n1 = parse_from_string("3.14").unwrap();
        assert_eq!(n1.to_string(), "3.14\n");

        let n2 = parse_from_string("3.14e2").unwrap();
        assert_eq!(n2.to_string(), "314\n");

        let n3 = parse_from_string("3.14e-1").unwrap();
        assert_eq!(n3.to_string(), "0.314\n");
    }

    #[test]
    fn test_complex_literal() {
        let n1 = parse_from_string("3+4i").unwrap();
        assert_eq!(n1.to_string(), "3+4i\n");

        let n2 = parse_from_string("0+2i").unwrap();
        assert_eq!(n2.to_string(), "0+2i\n");

        let n3 = parse_from_string("5i").unwrap();
        assert_eq!(n3.to_string(), "0+5i\n");

        let n4 = parse_from_string("1.414+2.718i").unwrap();
        assert_eq!(n4.to_string(), "1.414+2.718i\n");

        let n5 = parse_from_string("3.14i").unwrap();
        assert_eq!(n5.to_string(), "0+3.14i\n");

        let n6 = parse_from_string("3.14e2i").unwrap();
        assert_eq!(n6.to_string(), "0+314i\n");

        let n7 = parse_from_string("3.14e-1i").unwrap();
        assert_eq!(n7.to_string(), "0+0.314i\n");
    }

    #[test]
    fn test_bit_literal() {
        // todo::
        // let n1 = parse_from_string("16'x08cd").unwrap();
        // assert_eq!(n1.to_string(), "16'x08cd\n");
        //
        // let n2 = parse_from_string("8'b10000001").unwrap();
        // assert_eq!(n2.to_string(), "8'x81\n");
    }

    #[test]
    fn test_boolean_literal() {
        let n1 = parse_from_string("true").unwrap();
        assert_eq!(n1.to_string(), "true\n");

        let n2 = parse_from_string("false").unwrap();
        assert_eq!(n2.to_string(), "false\n");
    }

    #[test]
    fn test_char_literal() {
        let n1 = parse_from_string("'a'").unwrap();
        assert_eq!(n1.to_string(), "'a'\n");

        let n2 = parse_from_string("'文'").unwrap();
        assert_eq!(n2.to_string(), "'文'\n");

        // todo:: 测试转义字符，转义希腊字符
        // todo:: 测试 Unicode
    }

    #[test]
    fn test_general_string_literal() {
        let n1 = parse_from_string("\"abc\"").unwrap();
        assert_eq!(n1.to_string(), "\"abc\"\n");

        let n2 = parse_from_string("\"中文🐱\"").unwrap();
        assert_eq!(n2.to_string(), "\"中文🐱\"\n");

        // 测试多行文本
        let n3 = parse_from_string("\"foo\nbar\n  baz\"").unwrap();
        assert_eq!(n3.to_string(), "\"foo\nbar\n  baz\"\n");

        // todo:: 测试转义字符
    }

    #[test]
    fn test_template_string_literal() {
        // todo::
    }

    #[test]
    fn test_hash_string_literal() {
        let n1 = parse_from_string("#abc").unwrap();
        assert_eq!(n1.to_string(), "#abc\n");

        let n2 = parse_from_string("#foo_bar").unwrap();
        assert_eq!(n2.to_string(), "#foo_bar\n");

        // todo:: 添加中文的支持
        // let n3 = parse_from_string("#中文🐱").unwrap();
        // assert_eq!(n3.to_string(), "#中文🐱\n");
    }

    #[test]
    fn test_named_operator_string_literal() {
        let n1 = parse_from_string(":abc:").unwrap();
        assert_eq!(n1.to_string(), ":abc:\n");

        let n2 = parse_from_string(":foo_bar:").unwrap();
        assert_eq!(n2.to_string(), ":foo_bar:\n");

        // todo:: 添加中文的支持
        // let n3 = parse_from_string(":中文🐱:").unwrap();
        // assert_eq!(n3.to_string(), ":中文🐱:\n");
    }

    // primary expressions

    #[test]
    fn test_prefix_identifier() {
        let n1 = parse_from_string("!foo").unwrap();
        assert_eq!(n1.to_string(), "!foo\n");

        let n2 = parse_from_string("!foo::bar").unwrap();
        assert_eq!(n2.to_string(), "!foo::bar\n");
    }

    #[test]
    fn test_tuple() {
        let n1 = parse_from_string("(123,)").unwrap(); // 括号内的逗号不能省略
        assert_eq!(n1.to_string(), "(123,)\n");

        // 多个元素
        let n2 = parse_from_string("(123,1.732)").unwrap();
        assert_eq!(n2.to_string(), "(123, 1.732,)\n");

        // 元素列表以逗号结尾
        let n3 = parse_from_string("(123,1.732,)").unwrap();
        assert_eq!(n3.to_string(), "(123, 1.732,)\n");

        // 空元组
        let n4 = parse_from_string("()").unwrap();
        assert_eq!(n4.to_string(), "()\n");

        // 带有省略号元素的元组
        let n5 = parse_from_string("(123,...)").unwrap();
        assert_eq!(n5.to_string(), "(123, ...,)\n");

        // 带有省略号标识符元素的元组
        let n6 = parse_from_string("(123,...abc)").unwrap();
        assert_eq!(n6.to_string(), "(123, ...abc,)\n");

        // 逗号结尾
        let n7 = parse_from_string("(123,...abc,)").unwrap();
        assert_eq!(n7.to_string(), "(123, ...abc,)\n");

        // 多行格式
        let n8 = parse_from_string(&trim_left_margin(
            "(
                123,
                456,
                789
            )",
        ))
        .unwrap();
        assert_eq!(n8.to_string(), "(123, 456, 789,)\n");
    }

    #[test]
    fn test_list() {
        let n1 = parse_from_string("[123]").unwrap();
        assert_eq!(n1.to_string(), "[123,]\n");

        // 元素列表以 `逗号` 结尾
        let n2 = parse_from_string("[123,]").unwrap();
        assert_eq!(n2.to_string(), "[123,]\n");

        // 多个元素
        let n3 = parse_from_string("[123,1.732]").unwrap();
        assert_eq!(n3.to_string(), "[123, 1.732,]\n");

        // 元素列表以逗号结尾
        let n4 = parse_from_string("[123,1.732,]").unwrap();
        assert_eq!(n4.to_string(), "[123, 1.732,]\n");

        // 空列表
        let n5 = parse_from_string("[]").unwrap();
        assert_eq!(n5.to_string(), "[]\n");

        // 带有省略号元素的列表
        let n6 = parse_from_string("[123,...]").unwrap();
        assert_eq!(n6.to_string(), "[123, ...,]\n");

        // 带有省略号标识符元素的列表
        let n7 = parse_from_string("[123,...abc]").unwrap();
        assert_eq!(n7.to_string(), "[123, ...abc,]\n");

        // 逗号结尾
        let n8 = parse_from_string("[123,...abc,]").unwrap();
        assert_eq!(n8.to_string(), "[123, ...abc,]\n");

        // 范围表达式的列表
        let n9 = parse_from_string("[1..10]").unwrap();
        assert_eq!(n9.to_string(), "[1..10,]\n");

        // 逗号结尾
        let n10 = parse_from_string("[1..10,]").unwrap();
        assert_eq!(n10.to_string(), "[1..10,]\n");

        // "省略了范围结束值的范围表达式" 的列表
        let n11 = parse_from_string("[1..]").unwrap();
        assert_eq!(n11.to_string(), "[1..,]\n");

        // 逗号结尾
        let n12 = parse_from_string("[1..,]").unwrap();
        assert_eq!(n12.to_string(), "[1..,]\n");

        // 一个元素，以及一个范围表达式的列表
        let n13 = parse_from_string("[1,3..10]").unwrap();
        assert_eq!(n13.to_string(), "[1, 3..10,]\n");

        // 一个元素，以及一个省略了结束值的范围表达式的列表
        let n14 = parse_from_string("[1,3..]").unwrap();
        assert_eq!(n14.to_string(), "[1, 3..,]\n");

        // 闭区间
        let n15 = parse_from_string("[1..=10]").unwrap();
        assert_eq!(n15.to_string(), "[1..=10,]\n");

        // 一个元素，以及一个闭区间范围表达式的列表
        let n16 = parse_from_string("[1,3..=9]").unwrap();
        assert_eq!(n16.to_string(), "[1, 3..=9,]\n");

        // 多行格式
        let n17 = parse_from_string(&trim_left_margin(
            "[
                123,
                456,
                789
            ]",
        ))
        .unwrap();
        assert_eq!(n17.to_string(), "[123, 456, 789,]\n");
    }

    #[test]
    fn test_map() {
        let n1 = parse_from_string("{name:\"foo\"}").unwrap();
        assert_eq!(
            n1.to_string(),
            trim_left_margin(
                "{
                    name: \"foo\"
                }
                "
            )
        );

        let n2 = parse_from_string("{x:10,y:20}").unwrap();
        assert_eq!(
            n2.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                }
                "
            )
        );

        // 以逗号结尾
        let n3 = parse_from_string("{x:10,y:20,}").unwrap();
        assert_eq!(
            n3.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                }
                "
            )
        );

        // 多行格式
        let n3 = parse_from_string(&trim_left_margin(
            "{
                x:10
                y:20
                z:30
            }",
        ))
        .unwrap();
        assert_eq!(
            n3.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                    z: 30
                }
                "
            )
        );

        // 多行格式带逗号
        let n4 = parse_from_string(&trim_left_margin(
            "{
                x:10,
                y:20,
                z:30,
            }",
        ))
        .unwrap();
        assert_eq!(
            n4.to_string(),
            trim_left_margin(
                "{
                    x: 10
                    y: 20
                    z: 30
                }
                "
            )
        );

        // 测试缺少 `value` 部分的
        let n5 = parse_from_string(&trim_left_margin(
            "{
                x
                y:20,
                z
            }",
        ))
        .unwrap();
        assert_eq!(
            n5.to_string(),
            trim_left_margin(
                "{
                    x
                    y: 20
                    z
                }
                "
            )
        );

        // 测试 `省略号表达式`
        let n6 = parse_from_string("{x, y:20, ...rest}").unwrap();
        assert_eq!(
            n6.to_string(),
            trim_left_margin(
                "{
                    x
                    y: 20
                    ...rest
                }
                "
            )
        );

        // 测试多行格式的 `省略号表达式`
        let n7 = parse_from_string(&trim_left_margin(
            "{
                x
                y:20,
                ...rest
            }",
        ))
        .unwrap();
        assert_eq!(
            n7.to_string(),
            trim_left_margin(
                "{
                    x
                    y: 20
                    ...rest
                }
                "
            )
        );
    }

    #[test]
    fn test_identifier() {
        let n1 = parse_from_string("foo").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Identifier(Identifier {
                    dirs: vec![],
                    name: "foo".to_string(),
                    generic_names: vec![],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(n1.to_string(), "foo\n");

        let n2 = parse_from_string("foo::bar").unwrap();
        assert_eq!(n2.to_string(), "foo::bar\n");

        let n3 = parse_from_string("foo::bar::baz").unwrap();
        assert_eq!(n3.to_string(), "foo::bar::baz\n");
    }

    #[test]
    fn test_sign() {
        //
    }

    #[test]
    fn test_anonymous_function() {
        let n1 = parse_from_string("fn (Int a, Boolean b) type String = 1 + 2").unwrap();
        assert_eq!(
            n1.to_string(),
            "fn (Int a, Boolean b) type String = (1 + 2)\n"
        );

        // 无返回类型
        let n2 = parse_from_string("fn (Int a, Boolean b) = 1 + 2 * 3").unwrap();
        assert_eq!(n2.to_string(), "fn (Int a, Boolean b) = (1 + (2 * 3))\n");

        // 无数据类型
        let n3 = parse_from_string("fn (a, b) = a + b").unwrap();
        assert_eq!(n3.to_string(), "fn (a, b) = (a + b)\n");

        // 单独一个参数
        let n4 = parse_from_string("fn (a) = a + 1").unwrap();
        assert_eq!(n4.to_string(), "fn (a) = (a + 1)\n");

        // 单独一个参数且省略参数列表的括号
        let n5 = parse_from_string("fn a = a + 1").unwrap();
        assert_eq!(n5.to_string(), "fn (a) = (a + 1)\n");

        // 函数体为表达式块
        let n5 = parse_from_string("fn a {a + 1}").unwrap();
        assert_eq!(
            n5.to_string(),
            trim_left_margin(
                "fn (a) {
                    (a + 1)
                }
                "
            )
        );

        // 函数体为多行表达式块
        let n6 = parse_from_string("fn(a,b){a+b\na-b}").unwrap();
        assert_eq!(
            n6.to_string(),
            trim_left_margin(
                "fn (a, b) {
                    (a + b)
                    (a - b)
                }
                "
            )
        );
    }

    // operating expressions

    #[test]
    fn test_slice_expression() {
        //
    }

    #[test]
    fn test_member_expression() {
        //
    }

    #[test]
    fn test_constructor_expression() {
        //
    }

    #[test]
    fn test_function_call_expression() {
        //
    }

    #[test]
    fn test_unary_expression() {
        //
    }

    #[test]
    fn test_binary_expression_additive() {
        let n1 = parse_from_string("1+2").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::BinaryExpression(
                    BinaryExpression {
                        operator: Token::Plus,
                        left: Box::new(Expression::Literal(Literal::Integer(Integer {
                            value: 1,
                            range: new_range()
                        }))),
                        right: Box::new(Expression::Literal(Literal::Integer(Integer {
                            value: 2,
                            range: new_range()
                        }))),
                        range: new_range()
                    }
                ))],
                range: new_range()
            })
        );
        assert_eq!(n1.to_string(), "(1 + 2)\n"); // Statement 以符号 '\n' 结尾

        let n2 = parse_from_string("1+2+3").unwrap();
        assert_eq!(n2.to_string(), "((1 + 2) + 3)\n");

        let n3 = parse_from_string("1.414+1.732").unwrap();
        assert_eq!(n3.to_string(), "(1.414 + 1.732)\n");

        // 测试复数和加法并存的情况
        let n4 = parse_from_string("3+4i+9i").unwrap();
        assert_eq!(n4.to_string(), "(3+4i + 0+9i)\n");
    }

    #[test]
    fn test_binary_expression_precedence() {
        let n1 = parse_from_string("1|2||3").unwrap();
        assert_eq!(n1.to_string(), "(1 | (2 || 3))\n");

        let n2 = parse_from_string("1||2&&3").unwrap();
        assert_eq!(n2.to_string(), "(1 || (2 && 3))\n");

        let n3 = parse_from_string("1&&2==3").unwrap();
        assert_eq!(n3.to_string(), "(1 && (2 == 3))\n");

        let n4 = parse_from_string("1==2>3").unwrap();
        assert_eq!(n4.to_string(), "(1 == (2 > 3))\n");

        let n5 = parse_from_string("1>2:bit_or:3").unwrap();
        assert_eq!(n5.to_string(), "(1 > (2 :bit_or: 3))\n");

        let n6 = parse_from_string("1:bit_and:2++3").unwrap();
        assert_eq!(n6.to_string(), "(1 :bit_and: (2 ++ 3))\n");

        let n7 = parse_from_string("1++2+3").unwrap();
        assert_eq!(n7.to_string(), "(1 ++ (2 + 3))\n");

        let n8 = parse_from_string("1+2*3").unwrap();
        assert_eq!(n8.to_string(), "(1 + (2 * 3))\n");

        let n9 = parse_from_string("1*2??3").unwrap();
        assert_eq!(n9.to_string(), "(1 * (2 ?? 3))\n");

        let n10 = parse_from_string("1??2>>3").unwrap();
        assert_eq!(n10.to_string(), "(1 ?? (2 >> 3))\n");

        let n11 = parse_from_string("1>>2&3").unwrap();
        assert_eq!(n11.to_string(), "(1 >> (2 & 3))\n");
    }

    #[test]
    fn test_binary_expression_parenthesized() {
        let n1 = parse_from_string("(123)").unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Literal(
                    Literal::Integer(Integer {
                        value: 123,
                        range: new_range()
                    })
                ))],
                range: new_range()
            })
        );
        assert_eq!(n1.to_string(), "123\n");

        let n2 = parse_from_string("(1+2)").unwrap();
        assert_eq!(n2.to_string(), "(1 + 2)\n");

        let n3 = parse_from_string("(1+2)*3").unwrap();
        assert_eq!(n3.to_string(), "((1 + 2) * 3)\n");
    }

    #[test]
    fn test_binary_expression_associativitye() {
        // 测试结合方向

        // 操作符 `+` 从左向右结合
        let n1 = parse_from_string("1+2+3").unwrap();
        assert_eq!(n1.to_string(), "((1 + 2) + 3)\n");

        // 操作符 `&` 从右向左结合
        let n2 = parse_from_string("1&2&3").unwrap();
        assert_eq!(n2.to_string(), "(1 & (2 & 3))\n");
    }

    // genernal expression

    #[test]
    fn test_do_expression() {
        let n1 = parse_from_string(
            "do {
                123
                abc
            }",
        )
        .unwrap();
        assert_eq!(
            n1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::BlockExpression(
                    BlockExpression {
                        is_explicit: true,
                        body: vec![
                            Expression::Literal(Literal::Integer(Integer {
                                value: 123,
                                range: new_range()
                            })),
                            Expression::Identifier(Identifier {
                                dirs: vec![],
                                name: "abc".to_string(),
                                generic_names: vec![],
                                range: new_range()
                            }),
                        ],
                        range: new_range()
                    }
                ))],
                range: new_range()
            })
        );

        assert_eq!(
            n1.to_string(),
            trim_left_margin(
                "do {
                    123
                    abc
                }\n"
            )
        );
    }

    #[test]
    fn test_let_expression() {
        // todo::
    }

    #[test]
    fn test_join_expression() {
        // todo::
    }

    #[test]
    fn test_if_expression() {
        // todo::
    }

    #[test]
    fn test_for_expression() {
        // todo::
    }

    #[test]
    fn test_each_expression() {
        // todo::
    }

    #[test]
    fn test_branch_expression() {
        // todo::
    }

    #[test]
    fn test_match_expression() {
        // todo::
    }

    // statements

    #[test]
    fn test_function_declaration_statement() {
        // todo::
    }
}
