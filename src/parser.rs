/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use crate::{
    ast::{Expression, Integer, Literal, Node, Program, Range, Statement},
    error::Error,
    token::{Token, TokenDetail},
};

pub fn parse(token_details: &[TokenDetail]) -> Result<Node, Error> {
    let program = parse_program(token_details)?;
    Ok(Node::Program(program))
}

// Program
//  : {Statement}
//  ;
fn parse_program(token_details: &[TokenDetail]) -> Result<Program, Error> {
    let mut token_details = token_details;
    let mut statements = Vec::<Statement>::new();

    loop {
        if token_details.len() == 0 {
            break;
        }
        let (statement, rest_token_details) = parse_statement(token_details)?;
        statements.push(statement);

        // 解析完所有 token detail 为止
        token_details = rest_token_details;
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
fn parse_statement(token_details: &[TokenDetail]) -> Result<(Statement, &[TokenDetail]), Error> {
    // statement 以 Token::NewLine 或者 EOF 结束
    if let Some(first) = token_details.first() {
        match first.token {
            Token::Function => {
                // 函数声明语句
                parse_function_declaration(token_details)
            }
            _ => {
                // 表达式语句
                parse_expression_statement(token_details)
            }
        }
    } else {
        Ok((Statement::EmptyStatement, token_details))
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
    token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    todo!()
}

// * ExpressionStatement
// *  : Expression ';'
// *  ;

fn parse_expression_statement(
    token_details: &[TokenDetail],
) -> Result<(Statement, &[TokenDetail]), Error> {
    // statement 以 Token::NewLine 或者 EOF 结束
    let (expression, rest) = parse_expression(token_details)?;
    let (is_ok, post_rest) = consume_token_new_line_if_possible(rest);
    if is_ok {
        Ok((Statement::Expression(expression), post_rest))
    } else {
        Err(Error::ParserError("expected statement ending symbol"))
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

fn parse_expression(token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    if let Some(first) = token_details.first() {
        match first.token {
            Token::Do => {
                // do 表达式
                todo!()
            }
            Token::Let => {
                // let 表达式
                todo!()
            }
            Token::For => {
                // for 表达式
                todo!()
            }
            Token::Branch => {
                // branch 表达式
                todo!()
            }
            Token::Match => {
                // match 表达式
                todo!()
            }
            Token::If => {
                // if 表达式
                todo!()
            }
            _ => {
                // 二元运算表达式的开始
                parse_pipe_expression(token_details)
            }
        }
    } else {
        Err(Error::ParserError("expected expression"))
    }
}

// * BlockStatement
// *  : '{' OptionalStatementList  '}'
// *  ;

fn parse_do_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 do 表达式，`do {...}`，即显式表达式块
    todo!()
}

fn parse_block_expression_if_possible(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 解析 `do {...}`, `{...}` 或者 `...`
    //
    // 解析优先级：
    // 1. 解析 `do` 表达式，即显式表达式块（如果存在的话）
    // 2. 解析 `{...}` 表达式块，即隠式表达式块（如果存在的话）
    // 3. 作为普通的表达式解析
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
    token_details: &[TokenDetail],
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
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // for let ... =/in ... {... next}
    todo!()
}

fn parse_branch_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // branch {...}
    todo!()
}

fn parse_match_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // match ... {...}
    todo!()
}

// * IfStatement
// *  : 'if' '(' Expression ')' Statement
// *  | 'if' '(' Expression ')' Statement 'else' Statement
// *  ;

fn parse_if_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // if ... then ... else ...
    todo!()
}

// * AssignmentExpression
// *  : LogicalOrExpression
// *  | LeftHandSideExpression AssignmentOperator AssignmentExpression
// *  ;

fn parse_pipe_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left | right
    let (left, rest) = parse_logic_or_expression(token_details)?;
    todo!()
}

// * LogicalOrExpression
// *  : LogicalAndExpression
// *  | LogicalOrExpression LOGICAL_OR LogicalAndExpression
// *  ;

fn parse_logic_or_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left || right
    todo!()
}

// * LogicalAndExpression
// *  : EqualityExpression
// *  | LogicalAndExpression LOGICAL_AND EqualityExpression
// *  ;

fn parse_logic_and_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left && right
    todo!()
}

// * EqualityExpression
// *  : RelationalExpression
// *  | EqualityExpression EQUALITY_OPERATOR RelationalExpression
// *  ;

fn parse_equality_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left == right, left != right
    todo!()
}

// * RelationalExpression
// *  : AdditiveExpression
// *  | RelationalExpression RELATIONAL_OPERATOR AdditiveExpression
// *  ;

fn parse_relational_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left > right, left >= right, left < right, left <= right
    todo!()
}

fn parse_forward_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left >> right
    todo!()
}

fn parse_named_operator_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left :bitOr: right
    todo!()
}

fn parse_concat_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left ++ right
    todo!()
}

// * AdditiveExpression
// *  : MultiplicativeExpression
// *  | AdditiveExpression ADDITIVE_OPERATOR MultiplicativeExpression

fn parse_additive_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left + right, left - right
    todo!()
}

// * MultiplicativeExpression
// *  : UnaryExpression
// *  | MultiplicativeExpression MULTIPLICATIVE_OPERATOR UnaryExpression

fn parse_multiplicative_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left * right, left / right
    todo!()
}

// * UnaryExpression
// *  : LeftHandSideExpression
// *  | ADDITIVE_OPERATOR UnaryExpression
// *  | LOGICAL_NOT UnaryExpression
// *  ;

fn parse_unwrap_or_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left ?? right
    todo!()
}

fn parse_combine_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // left & right
    // 结合方向：从右向左
    todo!()
}

fn parse_cast_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 object^
    todo!()
}

fn parse_negative_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 -object
    todo!()
}

fn parse_unwrap_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // 一元运算表达式 object?
    todo!()
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
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // foo()
    todo!()
}

// * MemberExpression
// *  : PrimaryExpression
// *  | MemberExpression '.' Identifier
// *  | MemberExpression '[' Expression ']'

fn parse_member_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // object.property, object["foo"]
    todo!()
}

// * ConstructorExpression
// *  : Identifier {...}
// *  ;

fn parse_constructor_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // object {name: vale, ...}
    todo!()
}

fn parse_primary_expression(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // identifier | literal | parenthesized
    match token_details.split_first() {
        Some((first, rest)) => match first.token {
            Token::LeftParen => {
                todo!()
            }
            Token::Identifier(_) => {
                todo!()
            }
            _ => {
                let (literal, post_rest) = parse_literal(token_details)?;
                Ok((Expression::Literal(literal), post_rest))
            }
        },
        None => Err(Error::ParserError("expected primary expression")),
    }
}

fn parse_parenthesized(
    token_details: &[TokenDetail],
) -> Result<(Expression, &[TokenDetail]), Error> {
    // parenthesized
    todo!()
}

fn parse_identifier(token_details: &[TokenDetail]) -> Result<(Expression, &[TokenDetail]), Error> {
    // identifier
    todo!()
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

fn parse_literal(token_details: &[TokenDetail]) -> Result<(Literal, &[TokenDetail]), Error> {
    // literal
    match token_details.split_first() {
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
        None => Err(Error::ParserError("expected literal")),
    }
}

fn is_token(expected: Token, token_details: &[TokenDetail]) -> bool {
    match token_details.first() {
        Some(first) => first.token == expected,
        None => false,
    }
}

fn consume_token(expected: Token, token_details: &[TokenDetail]) -> (bool, &[TokenDetail]) {
    match token_details.split_first() {
        Some((first, rest)) => (first.token == expected, rest),
        None => (false, token_details),
    }
}

fn move_forword(token_details: &[TokenDetail], count: usize) -> &[TokenDetail] {
    &token_details[count..]
}

fn consume_token_new_line_if_possible(
    source_token_details: &[TokenDetail],
) -> (bool, &[TokenDetail]) {
    match source_token_details.split_first() {
        Some((first, rest)) => {
            if first.token == Token::NewLine {
                (true, rest)
            } else {
                (false, source_token_details)
            }
        }
        None => (true, source_token_details),
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
    use crate::{ast::Node, error::Error, lexer};

    use super::parse;

    fn parse_from_string(text: &str) -> Result<Node, Error> {
        let token_details = lexer::tokenize(text)?;
        parse(&token_details)
    }

    #[test]
    fn test_integer_literal() {
        // let a1 = parse_from_string("123").unwrap();
        // println!("{:?}", a1);
    }
}
