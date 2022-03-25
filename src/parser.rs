/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use crate::{
    ast::{
        BinaryExpression, Bit, BlockExpression, Boolean, Char, Complex, Ellipsis, Expression,
        Float, GeneralString, HashString, Identifier, Integer, Interval, List, Literal,
        NamedOperator, Node, PrefixIdentifier, Program, Range, Statement, Tuple, UnaryExpression,
    },
    error::Error,
    token::{Token, TokenDetail},
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
        let post_new_lines = skip_new_lines(source_token_details);

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
//  | Expression
//  ;
fn parse_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    let first = &source_token_details[0];
    match first.token {
        Token::Function => {
            // 函数声明语句
            parse_function_declaration(source_token_details)
        }
        _ => {
            // 表达式语句
            parse_expression_statement(source_token_details)
        }
    }
}

// * FunctionDeclaration
// *  : 'function' IDENTIFIER '(' OptionalFormalParameterList ')' BlockStatement
// *  ;
//
// * FormalParameterList
// *  : Identifier
// *  | FormalParameterList ',' Identifier
// *  ;

fn parse_function_declaration(
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
    consume_new_line_token_if_exists(rest)
        .map(|post_rest| (Statement::Expression(expression), post_rest))
}

// Expression
//  : BlockExpression
//  |
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

fn parse_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    if let Some(first) = source_token_details.first() {
        match first.token {
            Token::Do => {
                // do 表达式
                parse_do_expression(source_token_details)
            }
            Token::Join => {
                // join 表达式
                parse_join_expression(source_token_details)
            }
            Token::Let => {
                // let 表达式
                parse_let_expression(source_token_details)
            }
            Token::For => {
                // for 表达式
                parse_for_expression(source_token_details)
            }
            Token::Each => {
                // each 表达式
                parse_each_expression(source_token_details)
            }
            Token::Branch => {
                // branch 表达式
                parse_branch_expression(source_token_details)
            }
            Token::Match => {
                // match 表达式
                parse_match_expression(source_token_details)
            }
            Token::If => {
                // if 表达式
                parse_if_expression(source_token_details)
            }
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

    let post_consume_token_right_brace =
        consume_token_ignore_new_lines(&Token::RightBrace, token_details)?;

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

// * VariableStatementInit
// *  : 'let' VariableDeclarationList
// *  ;
//
// * VariableStatement
// *  : 'let' VariableDeclarationList ';'
// *  ;
//
// * VariableDeclarationList
// *  : VariableDeclaration
// *  | VariableDeclarationList ',' VariableDeclaration

// * VariableDeclaration
// *  : Identifier OptionalVariableInitializer
// *  ;
//
// * VariableInitializer
// *  : SIMPLE_ASSIGN AssignmentExpression
// *  ;

fn parse_let_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // let left =/match right
    todo!()
}

// * ForStatement
// *  : 'for' '(' OptionalForStatementInit ';' OptionalExpression ';' OptionalExpression ')' Statement
// *  ;
//
// * ForStatementInit
// *  : VariableStatementInit
// *  | Expression
// *  ;

fn parse_for_expression(
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

// * IfStatement
// *  : 'if' '(' Expression ')' Statement
// *  | 'if' '(' Expression ')' Statement 'else' Statement
// *  ;

fn parse_if_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // if ... then ... else ...
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

// * CallOrMemberExpression
// *  : MemberExpression
// *  | CallExpression
// *  ;
//
// * CallExpression
// *  : Callee Arguments
// *  ;
// *
// * Callee
// *  : MemberExpression
// *  | CallExpression

fn parse_function_call_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // foo()
    // todo::
    parse_member_or_slice_expression(source_token_details)
}

// * MemberExpression
// *  : PrimaryExpression
// *  | MemberExpression '.' Identifier
// *  | MemberExpression '[' Expression ']'

fn parse_member_or_slice_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // object.property, object["foo"]
    // todo::
    parse_constructor_expression(source_token_details)
}

// * ConstructorExpression
// *  : Identifier {...}
// *  ;

fn parse_constructor_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // object {name: vale, ...}
    // todo::
    parse_primary_expression(source_token_details)
}

// PrimaryExpression
//  : List
//  | Tuple/Parenthesized
//  | Map
//  | Identifier
//  ;
fn parse_primary_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    match source_token_details.first() {
        Some(first) => match first.token {
            Token::Exclamation => {
                // 函数的前置调用
                parse_prefix_identifier(source_token_details)
            }
            Token::LeftBracket => parse_list(source_token_details),
            Token::LeftParen => parse_tuple_or_parenthesized(source_token_details),
            Token::LeftBrace => parse_map(source_token_details),
            Token::Identifier(_) => parse_identifier(source_token_details),
            _ => {
                let (literal, post_rest) = parse_literal(source_token_details)?;
                Ok((Expression::Literal(literal), post_rest))
            }
        },
        None => Err(Error::ParserError(
            "expected primary expression".to_string(),
        )),
    }
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
    // literal
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

fn parse_list(source_token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    // list
    //
    // e.g.
    // [123, 345, 567,]
    // [1..10]
    // [1..=9]
    // [1,3..10]

    let mut token_details = source_token_details;

    let mut expressions: Vec<Expression> = vec![];
    let mut is_expected_end = false;

    token_details = consume_token(&Token::LeftBracket, token_details)?; // 消除左中括号
    token_details = skip_new_lines(token_details); // 左中括号后面允许换行

    loop {
        token_details = match token_details.first() {
            Some(first) => {
                if let TokenDetail {
                    token: Token::RightBracket,
                    ..
                } = first
                {
                    // 找到右中括号，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前只允许右中括号
                        return Err(Error::ParserError(
                            "expected the right bracket symbol \"]\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `展开式`
                        if let TokenDetail {
                            token: Token::Ellipsis,
                            ..
                        } = first
                        {
                            // 当前是 `展开式`
                            let (ellipsis, post_rest) = continue_parse_ellipsis(token_details)?;
                            expressions.push(Expression::Ellipsis(ellipsis));
                            is_expected_end = true; // 设置标记，`展开式` 后面只能允许右中括号

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_rest) {
                                consume_token(&Token::Comma, post_rest)?
                            } else {
                                post_rest
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 解析普通表达式
                            let (expression, post_rest) = parse_expression(token_details)?;

                            let post_check_interval = if is_token(&Token::Interval, post_rest)
                                || is_token(&Token::IntervalInclusive, post_rest)
                            {
                                // 当前是范围表达式
                                let (
                                    is_inclusive,
                                    optional_end_expression,
                                    post_continue_parse_interval,
                                ) = continue_parse_interval(post_rest)?;

                                let interval_expression = Expression::Interval(Interval {
                                    is_inclusive,
                                    from: Box::new(expression),
                                    to: match optional_end_expression {
                                        Some(end_expression) => Some(Box::new(end_expression)),
                                        None => None,
                                    },
                                    range: new_range(),
                                });

                                is_expected_end = true; // 设置标记，`范围表达式` 后面只能允许右中括号

                                expressions.push(interval_expression);
                                post_continue_parse_interval
                            } else {
                                // 当前是普通表达式
                                expressions.push(expression);
                                post_rest
                            };

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_check_interval)
                            {
                                consume_token(&Token::Comma, post_check_interval)?
                            } else {
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
                    "expected the right paren symbol \")\"".to_string(),
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
    // (one,)
    // (one, two, )
    // (one, two, ...)
    // (one, two, ...rest)
    //
    // parenthesized:
    //
    // (expression)

    let mut token_details = source_token_details;

    let mut expressions: Vec<Expression> = vec![];
    let mut is_tuple = false;
    let mut is_expected_end = false;

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
                    // 找到右括号，退出循环
                    break;
                } else {
                    if is_expected_end {
                        // 当前只允许右括号
                        return Err(Error::ParserError(
                            "expected the right paren symbol \")\"".to_string(),
                        ));
                    } else {
                        // 先检查是否 `展开式`
                        if let TokenDetail {
                            token: Token::Ellipsis,
                            ..
                        } = first
                        {
                            // 当前是 `展开式`
                            let (ellipsis, post_rest) = continue_parse_ellipsis(token_details)?;
                            expressions.push(Expression::Ellipsis(ellipsis));
                            is_expected_end = true; // 设置标记，`展开式` 后面只能允许右括号

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_rest) {
                                consume_token(&Token::Comma, post_rest)?
                            } else {
                                post_rest
                            };

                            // 消除空行
                            let post_consume_new_lines = skip_new_lines(post_consume_comma);
                            post_consume_new_lines
                        } else {
                            // 当前是普通表达式
                            let (expression, post_rest) = parse_expression(token_details)?;
                            expressions.push(expression);

                            // 消除逗号
                            let post_consume_comma = if is_token(&Token::Comma, post_rest) {
                                is_tuple = true;
                                consume_token(&Token::Comma, post_rest)?
                            } else {
                                post_rest
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
// 返回 (`end` 是否闭区间, `end` 部分表达式, 剩余的 token)
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

    // 消除范围符号 ".."
    let post_consume_token_interval = consume_token(&operator_token, source_token_details)?;

    // 范围符号 ".." 后面允许换行
    let post_new_lines = skip_new_lines(post_consume_token_interval);

    match post_new_lines.first() {
        Some(TokenDetail { token, .. })
            if (*token == Token::Comma || *token == Token::RightBracket) =>
        {
            // 遇到了逗号或者右中括号，说明当前范围表达式缺省 `end` 部分。
            Ok((is_inclusive, None, post_new_lines))
        }
        _ => {
            // 解析 `end` 部分表达式
            let (end_expression, post_end_expression) = parse_expression(post_new_lines)?;
            Ok((is_inclusive, Some(end_expression), post_end_expression))
        }
    }
}

fn parse_map(source_token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    // map
    //
    // e.g.
    // {name: value, name, expression, ...}
    todo!()
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
            _ => Err(()),
        },
        _ => Err(()),
    }
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

fn consume_token_ignore_new_lines<'a>(
    expected: &Token,
    source_token_details: &'a [TokenDetail],
) -> Result<&'a [TokenDetail], Error> {
    let token_details = skip_new_lines(source_token_details);
    consume_token(expected, token_details)
}

fn consume_new_line_token_if_exists(
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

    fn parse_from_string(text: &str) -> Result<Node, Error> {
        let token_details = lexer::tokenize(text)?;
        parse(&token_details)
    }

    // literal

    #[test]
    fn test_integer_literal() {
        let a1 = parse_from_string("123").unwrap();
        assert_eq!(
            a1,
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

        assert_eq!(a1.to_string(), "123\n"); // Statement 以符号 '\n' 结尾
    }

    #[test]
    fn test_float_literal() {
        let a1 = parse_from_string("3.14").unwrap();
        assert_eq!(a1.to_string(), "3.14\n");

        let a2 = parse_from_string("3.14e2").unwrap();
        assert_eq!(a2.to_string(), "314\n");

        let a3 = parse_from_string("3.14e-1").unwrap();
        assert_eq!(a3.to_string(), "0.314\n");
    }

    #[test]
    fn test_complex_literal() {
        let a1 = parse_from_string("3+4i").unwrap();
        assert_eq!(
            a1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Literal(
                    Literal::Complex(Complex {
                        real: 3.0,
                        imaginary: 4.0,
                        range: new_range()
                    })
                ))],
                range: new_range()
            })
        );
        assert_eq!(a1.to_string(), "3+4i\n");

        let a2 = parse_from_string("0+2i").unwrap();
        assert_eq!(a2.to_string(), "0+2i\n");

        let a3 = parse_from_string("5i").unwrap();
        assert_eq!(a3.to_string(), "0+5i\n");

        let a4 = parse_from_string("1.414+2.718i").unwrap();
        assert_eq!(a4.to_string(), "1.414+2.718i\n");

        let a5 = parse_from_string("3.14i").unwrap();
        assert_eq!(a5.to_string(), "0+3.14i\n");

        let a6 = parse_from_string("3.14e2i").unwrap();
        assert_eq!(a6.to_string(), "0+314i\n");

        let a7 = parse_from_string("3.14e-1i").unwrap();
        assert_eq!(a7.to_string(), "0+0.314i\n");
    }

    #[test]
    fn test_bit_literal() {
        // todo::
        //         let a1 = parse_from_string("16'x08cd").unwrap();
        //         assert_eq!(a1.to_string(), "16'x08cd\n");
        //
        //         let a2 = parse_from_string("8'b10000001").unwrap();
        //         assert_eq!(a2.to_string(), "8'x81\n");
    }

    #[test]
    fn test_boolean_literal() {
        // todo::
    }

    #[test]
    fn test_char_literal() {
        // todo::
    }

    #[test]
    fn test_general_string_literal() {
        // todo::
    }

    #[test]
    fn test_template_string_literal() {
        // todo::
    }

    #[test]
    fn test_hash_string_literal() {
        // todo::
    }

    #[test]
    fn test_named_operator_string_literal() {
        // todo::
    }

    // primary expressions

    #[test]
    fn test_identifier() {
        let a1 = parse_from_string("foo").unwrap();
        assert_eq!(
            a1,
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
        assert_eq!(a1.to_string(), "foo\n");

        let a2 = parse_from_string("foo::bar").unwrap();
        assert_eq!(
            a2,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Identifier(Identifier {
                    dirs: vec!["foo".to_string()],
                    name: "bar".to_string(),
                    generic_names: vec![],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a2.to_string(), "foo::bar\n");

        let a3 = parse_from_string("foo::bar::baz").unwrap();
        assert_eq!(a3.to_string(), "foo::bar::baz\n");
    }

    #[test]
    fn test_prefix_identifier() {
        let a1 = parse_from_string("!foo").unwrap();
        assert_eq!(
            a1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::PrefixIdentifier(
                    PrefixIdentifier {
                        identifier: Identifier {
                            dirs: vec![],
                            name: "foo".to_string(),
                            generic_names: vec![],
                            range: new_range()
                        },
                        range: new_range()
                    }
                ))],
                range: new_range()
            })
        );
        assert_eq!(a1.to_string(), "!foo\n");

        let a2 = parse_from_string("!foo::bar").unwrap();
        assert_eq!(a2.to_string(), "!foo::bar\n");
    }

    #[test]
    fn test_tuple() {
        let a1 = parse_from_string("(123,)").unwrap();
        assert_eq!(
            a1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Tuple(Tuple {
                    elements: vec![Expression::Literal(Literal::Integer(Integer {
                        value: 123,
                        range: new_range()
                    }))],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a1.to_string(), "(123,)\n");

        // 多个元素
        let a2 = parse_from_string("(123,1.732)").unwrap();
        assert_eq!(
            a2,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Tuple(Tuple {
                    elements: vec![
                        Expression::Literal(Literal::Integer(Integer {
                            value: 123,
                            range: new_range()
                        })),
                        Expression::Literal(Literal::Float(Float {
                            value: 1.732,
                            range: new_range()
                        }))
                    ],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a2.to_string(), "(123, 1.732,)\n");

        // 元素列表以逗号结尾
        let a3 = parse_from_string("(123,1.732,)").unwrap();
        assert_eq!(a3.to_string(), "(123, 1.732,)\n");

        // 空元组
        let a4 = parse_from_string("()").unwrap();
        assert_eq!(
            a4,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Tuple(Tuple {
                    elements: vec![],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a4.to_string(), "()\n");

        // 带有省略号元素的元组
        let a5 = parse_from_string("(123,...)").unwrap();
        assert_eq!(
            a5,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Tuple(Tuple {
                    elements: vec![
                        Expression::Literal(Literal::Integer(Integer {
                            value: 123,
                            range: new_range()
                        })),
                        Expression::Ellipsis(Ellipsis {
                            name: None,
                            range: new_range()
                        })
                    ],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a5.to_string(), "(123, ...,)\n");

        // 带有省略号标识符元素的元组
        let a6 = parse_from_string("(123,...abc)").unwrap();
        assert_eq!(
            a6,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Tuple(Tuple {
                    elements: vec![
                        Expression::Literal(Literal::Integer(Integer {
                            value: 123,
                            range: new_range()
                        })),
                        Expression::Ellipsis(Ellipsis {
                            name: Some("abc".to_string()),
                            range: new_range()
                        })
                    ],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a6.to_string(), "(123, ...abc,)\n");
    }

    #[test]
    fn test_list() {
        let a1 = parse_from_string("[123]").unwrap();
        assert_eq!(
            a1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::List(List {
                    elements: vec![Expression::Literal(Literal::Integer(Integer {
                        value: 123,
                        range: new_range()
                    }))],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a1.to_string(), "[123,]\n");

        // 元素列表以 `逗号` 结尾
        let a2 = parse_from_string("[123,]").unwrap();
        assert_eq!(a2.to_string(), "[123,]\n");

        // 多个元素
        let a3 = parse_from_string("[123,1.732]").unwrap();
        assert_eq!(
            a3,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::List(List {
                    elements: vec![
                        Expression::Literal(Literal::Integer(Integer {
                            value: 123,
                            range: new_range()
                        })),
                        Expression::Literal(Literal::Float(Float {
                            value: 1.732,
                            range: new_range()
                        }))
                    ],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a3.to_string(), "[123, 1.732,]\n");

        // 元素列表以逗号结尾
        let a4 = parse_from_string("[123,1.732,]").unwrap();
        assert_eq!(a4.to_string(), "[123, 1.732,]\n");

        // 空列表
        let a5 = parse_from_string("[]").unwrap();
        assert_eq!(
            a5,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::List(List {
                    elements: vec![],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a5.to_string(), "[]\n");

        // 带有省略号元素的列表
        let a6 = parse_from_string("[123,...]").unwrap();
        assert_eq!(
            a6,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::List(List {
                    elements: vec![
                        Expression::Literal(Literal::Integer(Integer {
                            value: 123,
                            range: new_range()
                        })),
                        Expression::Ellipsis(Ellipsis {
                            name: None,
                            range: new_range()
                        })
                    ],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a6.to_string(), "[123, ...,]\n");

        // 带有省略号标识符元素的列表
        let a7 = parse_from_string("[123,...abc]").unwrap();
        assert_eq!(a7.to_string(), "[123, ...abc,]\n");

        // 范围表达式的列表
        let a8 = parse_from_string("[1..10]").unwrap();
        assert_eq!(
            a8,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::List(List {
                    elements: vec![Expression::Interval(Interval {
                        is_inclusive: false,
                        from: Box::new(Expression::Literal(Literal::Integer(Integer {
                            value: 1,
                            range: new_range()
                        }))),
                        to: Some(Box::new(Expression::Literal(Literal::Integer(Integer {
                            value: 10,
                            range: new_range()
                        })))),
                        range: new_range()
                    })],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a8.to_string(), "[1..10,]\n");

        // "省略了范围结束值的范围表达式" 的列表
        let a9 = parse_from_string("[1..]").unwrap();
        assert_eq!(
            a9,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::List(List {
                    elements: vec![Expression::Interval(Interval {
                        is_inclusive: false,
                        from: Box::new(Expression::Literal(Literal::Integer(Integer {
                            value: 1,
                            range: new_range()
                        }))),
                        to: None,
                        range: new_range()
                    })],
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a9.to_string(), "[1..,]\n");

        // 闭区间
        todo!();

        // 带有一个元素以及一个范围表达式的列表
        let a10 = parse_from_string("[1,3..9,]").unwrap();
        assert_eq!(a10.to_string(), "[1, 3..9,]\n");

        let a11 = parse_from_string("[1,3..,]").unwrap();
        assert_eq!(a11.to_string(), "[1, 3..,]\n");
    }

    // operating expressions

    #[test]
    fn test_binary_expression_additive() {
        let a1 = parse_from_string("1+2").unwrap();
        assert_eq!(
            a1,
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

        assert_eq!(a1.to_string(), "(1 + 2)\n"); // Statement 以符号 '\n' 结尾

        let a2 = parse_from_string("1+2+3").unwrap();
        assert_eq!(a2.to_string(), "((1 + 2) + 3)\n");

        let a3 = parse_from_string("1.414+1.732").unwrap();
        assert_eq!(a3.to_string(), "(1.414 + 1.732)\n");

        let a4 = parse_from_string("3+4i+9i").unwrap();
        assert_eq!(a4.to_string(), "(3+4i + 0+9i)\n");
    }

    #[test]
    fn test_binary_expression_precedence() {
        // 测试优先级
        let a1 = parse_from_string("1|2||3").unwrap();
        assert_eq!(a1.to_string(), "(1 | (2 || 3))\n");

        let a2 = parse_from_string("1||2&&3").unwrap();
        assert_eq!(a2.to_string(), "(1 || (2 && 3))\n");

        let a3 = parse_from_string("1&&2==3").unwrap();
        assert_eq!(a3.to_string(), "(1 && (2 == 3))\n");

        let a4 = parse_from_string("1==2>3").unwrap();
        assert_eq!(a4.to_string(), "(1 == (2 > 3))\n");

        let a5 = parse_from_string("1>2>>3").unwrap();
        assert_eq!(a5.to_string(), "(1 > (2 >> 3))\n");

        let a6 = parse_from_string("1>>2:bit_or:3").unwrap();
        assert_eq!(a6.to_string(), "(1 >> (2 :bit_or: 3))\n");

        let a7 = parse_from_string("1:bit_and:2++3").unwrap();
        assert_eq!(a7.to_string(), "(1 :bit_and: (2 ++ 3))\n");

        let a8 = parse_from_string("1++2+3").unwrap();
        assert_eq!(a8.to_string(), "(1 ++ (2 + 3))\n");

        let a9 = parse_from_string("1+2*3").unwrap();
        assert_eq!(a9.to_string(), "(1 + (2 * 3))\n");

        let a10 = parse_from_string("1*2??3").unwrap();
        assert_eq!(a10.to_string(), "(1 * (2 ?? 3))\n");

        let a11 = parse_from_string("1??2&3").unwrap();
        assert_eq!(a11.to_string(), "(1 ?? (2 & 3))\n");
    }

    #[test]
    fn test_parenthesized_expression() {
        let a1 = parse_from_string("(123)").unwrap();
        assert_eq!(
            a1,
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

        assert_eq!(a1.to_string(), "123\n");

        let a2 = parse_from_string("(1+2)").unwrap();
        assert_eq!(a2.to_string(), "(1 + 2)\n");

        let a3 = parse_from_string("(1+2)*3").unwrap();
        assert_eq!(a3.to_string(), "((1 + 2) * 3)\n");
    }

    #[test]
    fn test_binary_expression_associativitye() {
        // 从结合方向
        // 操作符 `&` 从右向左结合
        let a1 = parse_from_string("1&2&3").unwrap();
        assert_eq!(a1.to_string(), "(1 & (2 & 3))\n");
    }

    // genernal expression

    #[test]
    fn test_do_expression() {
        let a1 = parse_from_string(
            "do {
                123
                abc
            }",
        )
        .unwrap();
        assert_eq!(
            a1,
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

        assert_eq!(a1.to_string(), "{\n123\nabc\n}\n");
    }

    // statement

    // operating expressions
    //
    //     #[test]
    //     fn test_display_binary_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_unary_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_function_call_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_member_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_slice_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_constructor_expression() {
    //         // todo::
    //     }
    //
    //     // general expressions
    //
    //     #[test]
    //     fn test_display_block_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_let_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_for_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_branch_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_match_expression() {
    //         // todo::
    //     }
    //
    //     #[test]
    //     fn test_display_if_expression() {
    //         let e1 = Expression::IfExpression(IfExpression {
    //             condition: Box::new(Expression::Literal(Literal::Boolean(Boolean {
    //                 value: true,
    //                 range: new_range(),
    //             }))),
    //             consequent: Box::new(Expression::Literal(Literal::Integer(Integer {
    //                 value: 1,
    //                 range: new_range(),
    //             }))),
    //             alternate: Some(Box::new(Expression::Literal(Literal::Integer(Integer {
    //                 value: 2,
    //                 range: new_range(),
    //             })))),
    //             range: new_range(),
    //         });
    //
    //         assert_eq!(e1.to_string(), "if true then 1 else 2")
    //     }

    // statements

    // #[test]
    // fn test_display_function_declaration_statement() {
    //             let s1 = Statement::FunctionDeclaration(FunctionDeclaration {
    //                 name: "inc".to_string(),
    //                 params: vec![(
    //                     Identifier {
    //                         dirs: vec![],
    //                         name: "Int".to_string(),
    //                         generic_names: vec![],
    //                         range: new_range(),
    //                     },
    //                     Expression::Identifier(Identifier {
    //                         dirs: vec![],
    //                         name: "a".to_string(),
    //                         generic_names: vec![],
    //                         range: new_range(),
    //                     }),
    //                 )],
    //                 return_type: Identifier {
    //                     dirs: vec![],
    //                     name: "Int".to_string(),
    //                     generic_names: vec![],
    //                     range: new_range(),
    //                 },
    //                 body: Box::new(Expression::BinaryExpression(BinaryExpression {
    //                     operator: Token::Plus,
    //                     left: Box::new(Expression::Identifier(Identifier {
    //                         dirs: vec![],
    //                         name: "a".to_string(),
    //                         range: new_range(),
    //                     })),
    //                     right: Box::new(Expression::Literal(Literal::Integer(Integer {
    //                         value: 1,
    //                         range: new_range(),
    //                     }))),
    //                     range: new_range(),
    //                 })),
    //                 range: new_range(),
    //             });
    //
    //             assert_eq!(s1.to_string(), "function inc (Int a) type Int = (a + 1)\n")
    // }

    // nodes

    //     #[test]
    //     fn test_display_node_program() {
    //         let p1 = Node::Program(Program {
    //             body: vec![Statement::Expression(Expression::Literal(
    //                 Literal::Integer(Integer {
    //                     value: 123,
    //                     range: new_range(),
    //                 }),
    //             ))],
    //             range: new_range(),
    //         });
    //
    //         assert_eq!(p1.to_string(), "123\n");
    //     }
}
