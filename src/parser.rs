/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use crate::{
    ast::{
        BinaryExpression, BlockExpression, Expression, Identifier, Integer, Literal, Node,
        PrefixIdentifier, Program, Range, Statement, UnaryExpression,
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
        if token_details.len() == 0 {
            break;
        }

        let (statement, post_rest) = parse_statement(token_details)?;
        statements.push(statement);

        // 再解析剩余的 token，直到解析完所有 token 为止
        token_details = post_rest;
    }

    Ok(Program {
        body: statements,
        range: new_range(),
    })
}

// Statement
//  : EmptyStatement
//  | FunctionDeclaration
//  | Expression
//  ;
fn parse_statement(
    source_token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    // 消除前导的空行
    let token_details = skip_new_lines(source_token_details);

    if let Some(first) = token_details.first() {
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
    } else {
        // 当一个程序的有效源码是空的时候，解析而得的语法树就只有一个 EmptyStatement
        Ok((Statement::EmptyStatement, source_token_details))
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

fn parse_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    let token_details = source_token_details;
    if let Some(first) = token_details.first() {
        match first.token {
            Token::Do => {
                // do 表达式
                parse_do_expression(token_details)
            }
            Token::Let => {
                // let 表达式
                parse_let_expression(token_details)
            }
            Token::For => {
                // for 表达式
                parse_for_expression(token_details)
            }
            Token::Branch => {
                // branch 表达式
                parse_branch_expression(token_details)
            }
            Token::Match => {
                // match 表达式
                parse_match_expression(token_details)
            }
            Token::If => {
                // if 表达式
                parse_if_expression(token_details)
            }
            _ => {
                // 二元运算表达式的开始
                parse_pipe_expression(token_details)
            }
        }
    } else {
        Err(Error::ParserError("expected expression".to_string()))
    }
}

// DoStatement
//  : 'do' BlockStatement
//  ;
fn parse_do_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 do 表达式，`do {...}`，即显式表达式块
    let mut token_details = source_token_details;

    token_details = consume_token(&Token::Do, token_details)?;

    // do 关键字后面允许换行
    token_details = skip_new_lines(token_details);

    parse_block_expression(token_details)
}

// BlockStatement
//  : '{' StatementList '}'
//  ;
fn parse_block_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析隠式表达式块 `{...}`
    // 注意隠式表达式块仅存在某些关键字后面，比如 `do`、`then` 等，而不能单独存在，
    // 当一对花括号单独存在时，会被解析为 Map。
    let mut token_details = source_token_details;

    token_details = consume_token(&Token::LeftBrace, token_details)?;

    let mut expressions: Vec<Expression> = vec![];

    loop {
        // 左花括号 '{' 后面允许换行
        // 表达式之间也是以换行分隔
        token_details = skip_new_lines(token_details);
        let (expression, post_rest) = parse_expression(token_details)?;
        expressions.push(expression);

        token_details = post_rest;

        if is_token_ignore_new_lines(&Token::RightBrace, token_details) {
            break;
        }
    }

    token_details = consume_token_ignore_new_lines(&Token::RightBrace, token_details)?;

    Ok((
        Expression::BlockExpression(BlockExpression {
            is_explicit: true,
            body: expressions,
            range: new_range(),
        }),
        token_details,
    ))
}

fn parse_expression_block_or_single_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 `do {...}`, `{...}` 或者 `...`
    //
    // 解析优先级：
    // 1. 解析 `do` 表达式，即显式表达式块（如果存在的话）
    // 2. 解析 `{...}` 表达式块，即隠式表达式块（如果存在的话）
    // 3. 作为普通的表达式解析
    match source_token_details.first() {
        Some(first) => match first.token {
            Token::Do => parse_do_expression(source_token_details),
            Token::LeftBrace => parse_block_expression(source_token_details),
            _ => parse_expression(source_token_details),
        },
        None => Err(Error::ParserError(
            "expected an expression or an expression block".to_string(),
        )),
    }
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
    // for let ... =/in ... {... next}
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
    let (mut left, post_rest) = next_parse_function(token_details)?;
    token_details = post_rest;

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

        token_details = consume_token(operator_token, token_details)?;

        // 二元运算符后面允许换行
        token_details = skip_new_lines(token_details);

        let (right, post_rest) = next_parse_function(token_details)?;
        token_details = post_rest;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
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
    let (mut left, post_rest) = next_parse_function(token_details)?;
    token_details = post_rest;

    if is_token(operator_token, token_details) {
        token_details = consume_token(operator_token, token_details)?;

        // 二元运算符后面允许换行
        token_details = skip_new_lines(token_details);

        let (right, post_rest) = parse_expression(token_details)?;
        token_details = post_rest;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
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
        parse_forward_expression,
        source_token_details,
    )
}

fn parse_forward_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left >> right
    parse_binary_expression(
        &vec![Token::Forward],
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
    let (mut left, post_rest) = parse_concat_expression(token_details)?;
    token_details = post_rest;

    if let Some(TokenDetail {
        token: named_operator_token @ Token::NamedOperator(_),
        ..
    }) = token_details.first()
    {
        token_details = consume_token(named_operator_token, token_details)?;

        // 二元运算符后面允许换行
        token_details = skip_new_lines(token_details);

        let (right, post_rest) = parse_concat_expression(token_details)?;
        token_details = post_rest;

        let expression = Expression::BinaryExpression(BinaryExpression {
            operator: named_operator_token.clone(),
            left: Box::new(left),
            right: Box::new(right),
            range: new_range(),
        });

        left = expression;
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
        parse_unwrap_or_expression,
        source_token_details,
    )
}

fn parse_unwrap_or_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left ?? right
    parse_binary_expression(
        &vec![Token::UnwrapOr],
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
    let mut token_details = source_token_details;
    let (left, post_rest) = parse_negative_expression(token_details)?;
    token_details = post_rest;

    if is_token(&Token::Cast, token_details) {
        token_details = consume_token(&Token::Cast, token_details)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Cast,
                operand: Box::new(left),
                range: new_range(),
            }),
            token_details,
        ))
    } else {
        Ok((left, token_details))
    }
}

fn parse_negative_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 -object
    let mut token_details = source_token_details;

    if is_token(&Token::Minus, token_details) {
        token_details = consume_token(&Token::Cast, token_details)?;
        let (left, post_rest) = parse_unwrap_expression(token_details)?;
        token_details = post_rest;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Minus,
                operand: Box::new(left),
                range: new_range(),
            }),
            token_details,
        ))
    } else {
        parse_unwrap_expression(source_token_details)
    }
}

fn parse_unwrap_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 object?
    let mut token_details = source_token_details;
    let (left, post_rest) = parse_function_call_expression(token_details)?;
    token_details = post_rest;

    if is_token(&Token::Unwrap, token_details) {
        token_details = consume_token(&Token::Unwrap, token_details)?;

        Ok((
            Expression::UnaryExpression(UnaryExpression {
                operator: Token::Unwrap,
                operand: Box::new(left),
                range: new_range(),
            }),
            token_details,
        ))
    } else {
        Ok((left, token_details))
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
    parse_member_expression(source_token_details)
}

// * MemberExpression
// *  : PrimaryExpression
// *  | MemberExpression '.' Identifier
// *  | MemberExpression '[' Expression ']'

fn parse_member_expression(
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

fn parse_primary_expression(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // identifier | literal |
    match source_token_details.first() {
        Some(first) => match first.token {
            Token::Exclamation => {
                // 函数的前置调用
                parse_prefix_identifier(source_token_details)
            }
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
    let mut token_details = source_token_details;
    token_details = consume_token(&Token::Exclamation, token_details)?;

    let (identifier, post_rest) = parse_identifier_object(token_details)?;
    token_details = post_rest;

    Ok((
        Expression::PrefixIdentifier(PrefixIdentifier {
            identifier: identifier,
            range: new_range(),
        }),
        token_details,
    ))
}

fn parse_identifier(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // identifier
    //
    // One::Two::Three::Name
    let mut token_details = source_token_details;

    let (identifier, post_rest) = parse_identifier_object(token_details)?;
    token_details = post_rest;

    Ok((Expression::Identifier(identifier), token_details))
}

fn parse_identifier_object(
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
                Some((first, post_separator_rest)) if first.token == Token::Separator => {
                    // 检测到 namespace path 分隔符 `::`
                    token_details = post_separator_rest;

                    if let Some((
                        TokenDetail {
                            token: Token::Identifier(name),
                            ..
                        },
                        post_identifier_rest,
                    )) = token_details.split_first()
                    {
                        // 检测到一个 identifier
                        names.push(name.clone());
                        post_identifier_rest
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
        //if names.len() == 1 {
        //     Ok((
        //         Identifier {
        //             dirs: vec![],
        //             name: names[0].clone(),
        //             range: new_range(),
        //         },
        //         token_details,
        //     ))
        // } else {
        let len = names.len();
        Ok((
            Identifier {
                dirs: names[..len - 1].iter().map(|n| n.clone()).collect(),
                name: names[len - 1].clone(),
                range: new_range(),
            },
            token_details,
        ))
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
//  | Tuple/Parenthesized
//  | Map
//  ;

fn parse_literal(source_token_details: &[TokenDetail]) -> Result<(Literal, &[TokenDetail]), Error> {
    // literal
    match source_token_details.split_first() {
        Some((first, rest)) => match first.token {
            Token::Integer(v) => Ok((
                Literal::Integer(Integer {
                    value: v,
                    range: new_range(),
                }),
                rest,
            )),
            _ => {
                todo!()
            }
        },
        None => Err(Error::ParserError("expected literal".to_string())),
    }
}

fn parse_parenthesized(
    source_token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // parenthesized
    todo!()
}

// 跳过空白的行，在 lexer 里产生的 Token 序列当中，有可能存在多行连续的空行，
// 在解析一个statement 之前，或者 expression 之间，需要消除这些空白的前导空行
fn skip_new_lines(source_token_details: &[TokenDetail]) -> &[TokenDetail] {
    let mut token_details = source_token_details;
    let mut count: usize = 0;

    loop {
        token_details = match token_details.split_first() {
            Some((first, rest)) if first.token == Token::NewLine => {
                count += 1;
                rest
            }
            _ => {
                break;
            }
        }
    }

    move_forword(source_token_details, count)
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

// fn is_token_identifier(source_token_details: &[TokenDetail]) -> bool {
//     match source_token_details.first() {
//         Some(TokenDetail {
//             token: Token::Identifier(_),
//             ..
//         }) => true,
//         _ => false,
//     }
// }

fn consume_token<'a>(
    expected: &Token,
    source_token_details: &'a [TokenDetail],
) -> Result<&'a [TokenDetail], Error> {
    match source_token_details.split_first() {
        Some((first, rest)) if &first.token == expected => Ok(rest),
        _ => Err(Error::ParserError(format!("expected symbol {}", expected))),
    }
}

fn consume_token_ignore_new_lines<'a>(
    expected: &Token,
    source_token_details: &'a [TokenDetail],
) -> Result<&'a [TokenDetail], Error> {
    let token_details = skip_new_lines(source_token_details);
    consume_token(expected, token_details)
}

fn move_forword(source_token_details: &[TokenDetail], count: usize) -> &[TokenDetail] {
    &source_token_details[count..]
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
                    "expected line ending symbol".to_string(),
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
            BinaryExpression, Expression, Identifier, Integer, Literal, Node, PrefixIdentifier,
            Program, Statement,
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
    fn test_additive_expression() {
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
    fn test_binary_expression_associativitye() {
        // 从结合方向
        // 操作符 `&` 从右向左结合
        let a1 = parse_from_string("1&2&3").unwrap();
        assert_eq!(a1.to_string(), "(1 & (2 & 3))\n");
    }

    #[test]
    fn test_identifier() {
        let a1 = parse_from_string("foo").unwrap();
        assert_eq!(
            a1,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::Identifier(Identifier {
                    dirs: vec![],
                    name: "foo".to_string(),
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
                    range: new_range()
                }))],
                range: new_range()
            })
        );
        assert_eq!(a2.to_string(), "foo::bar\n");

        let a3 = parse_from_string("foo::bar::baz").unwrap();
        assert_eq!(a3.to_string(), "foo::bar::baz\n");

        let a4 = parse_from_string("!foo").unwrap();
        assert_eq!(
            a4,
            Node::Program(Program {
                body: vec![Statement::Expression(Expression::PrefixIdentifier(
                    PrefixIdentifier {
                        identifier: Identifier {
                            dirs: vec![],
                            name: "foo".to_string(),
                            range: new_range()
                        },
                        range: new_range()
                    }
                ))],
                range: new_range()
            })
        );
        assert_eq!(a4.to_string(), "!foo\n");

        let a5 = parse_from_string("!foo::bar").unwrap();
        assert_eq!(a5.to_string(), "!foo::bar\n");
    }
}
