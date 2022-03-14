/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use std::char;

use crate::error::Error;
use crate::token::Location;
use crate::token::Token;
use crate::token::TokenType;

pub fn tokenize(program: &str) -> Result<Vec<Token>, Error> {
    let vec_char: Vec<char> = program.chars().collect();

    let mut chars = &vec_char[..];
    let mut tokens: Vec<Token> = vec![];

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    ' ' | '\t' => {
                        // skip whitespace
                        rest
                    }
                    '/' => {
                        if match_char('/', rest) {
                            // skip comment
                            skip_comment(chars) // "//..."
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Slash)); // "/"
                            rest
                        }
                    }
                    '\n' => {
                        add_token(&mut tokens, new_token(TokenType::NewLine)); // "\n"
                        rest
                    }
                    '\r' => {
                        if match_char('\n', rest) {
                            add_token(&mut tokens, new_token(TokenType::NewLine)); // "\r\n"
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::NewLine)); // "\r"
                            rest
                        }
                    }
                    '{' => {
                        add_token(&mut tokens, new_token(TokenType::LeftBrace)); // "{"
                        rest
                    }
                    '}' => {
                        add_token(&mut tokens, new_token(TokenType::RightBrace)); // "}"
                        rest
                    }
                    '=' => {
                        if match_char('=', rest) {
                            add_token(&mut tokens, new_token(TokenType::Equal)); // "=="
                            move_forword(rest, 1)
                        } else if match_char('>', rest) {
                            add_token(&mut tokens, new_token(TokenType::Arrow)); // "=>"
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Assign)); // "="
                            rest
                        }
                    }
                    '>' => {
                        if match_char('>', rest) {
                            add_token(&mut tokens, new_token(TokenType::Forward)); // ">>"
                            move_forword(rest, 1)
                        } else if match_char('=', rest) {
                            add_token(&mut tokens, new_token(TokenType::GreaterThanOrEqual)); // ">="
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::GreaterThan)); // ">"
                            rest
                        }
                    }
                    '|' => {
                        if match_char('|', rest) {
                            add_token(&mut tokens, new_token(TokenType::LogicOr)); // "||"
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Pipe)); // "|"
                            rest
                        }
                    }
                    '&' => {
                        if match_char('&', rest) {
                            add_token(&mut tokens, new_token(TokenType::LogicAnd)); // "&&"
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Combine)); // "&"
                            rest
                        }
                    }
                    '!' => {
                        if match_char('=', rest) {
                            add_token(&mut tokens, new_token(TokenType::NotEqual)); // "!="
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Exclamation)); // "!"
                            rest
                        }
                    }
                    '<' => {
                        if match_char('=', rest) {
                            add_token(&mut tokens, new_token(TokenType::LessThanOrEqual)); // "<="
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::LessThan)); // "<"
                            rest
                        }
                    }
                    '+' => {
                        if match_char('+', rest) {
                            add_token(&mut tokens, new_token(TokenType::Concat)); // "++"
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Plus)); // "+"
                            rest
                        }
                    }
                    '-' => {
                        add_token(&mut tokens, new_token(TokenType::Minus)); // "-"
                        rest
                    }
                    '*' => {
                        add_token(&mut tokens, new_token(TokenType::Asterisk)); // "*"
                        rest
                    }
                    '?' => {
                        if match_char('?', rest) {
                            add_token(&mut tokens, new_token(TokenType::UnwrapOr)); // "??"
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Unwrap)); // "?"
                            rest
                        }
                    }
                    '^' => {
                        add_token(&mut tokens, new_token(TokenType::Cast)); // "^"
                        rest
                    }
                    '.' => {
                        if match_chars(['.', '.'], rest) {
                            add_token(&mut tokens, new_token(TokenType::Ellipsis)); // "..."
                            move_forword(rest, 2)
                        } else if match_char('.', rest) {
                            add_token(&mut tokens, new_token(TokenType::Range)); // ".."
                            move_forword(rest, 1)
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Dot)); // "."
                            rest
                        }
                    }
                    '[' => {
                        add_token(&mut tokens, new_token(TokenType::LeftBracket)); // "["
                        rest
                    }
                    ']' => {
                        add_token(&mut tokens, new_token(TokenType::RightBracket)); // "]"
                        rest
                    }
                    '(' => {
                        add_token(&mut tokens, new_token(TokenType::LeftParen)); // "("
                        rest
                    }
                    ')' => {
                        add_token(&mut tokens, new_token(TokenType::RightParen)); // ")"
                        rest
                    }
                    '#' => {
                        add_token(&mut tokens, new_token(TokenType::Hash)); // "#"
                        rest
                    }
                    ':' => {
                        add_token(&mut tokens, new_token(TokenType::Colon)); // ":"
                        rest
                    }
                    ',' => {
                        add_token(&mut tokens, new_token(TokenType::Comma)); // ","
                        rest
                    }
                    _ => return Err(Error::LexerError("unexpected char")),
                };
            }
            None => break,
        };
    }

    Ok(tokens)
}

fn add_token(tokens: &mut Vec<Token>, token: Token) -> &mut Vec<Token> {
    tokens.push(token);
    tokens
}

fn new_token(token_type: TokenType) -> Token {
    Token {
        location: Location {
            file_id: 0,
            start: 0,
            end: 0,
        },
        token_type,
    }
}

fn match_char(expectd: char, chars: &[char]) -> bool {
    match chars.first() {
        Some(first_char) => *first_char == expectd,
        None => false,
    }
}

fn match_chars(expected: [char; 2], chars: &[char]) -> bool {
    match chars.split_first() {
        Some((first, rest)) => {
            if *first == expected[0] {
                match rest.first() {
                    Some(second) => *second == expected[1],
                    None => false,
                }
            } else {
                false
            }
        }
        None => false,
    }
}

fn skip_comment(chars: &[char]) -> &[char] {
    // 寻找 '\n'
    match chars.iter().position(|c| *c == '\n') {
        Some(index) => &chars[index..],
        None => &chars[chars.len()..],
    }
}

fn move_forword(chars: &[char], count: usize) -> &[char] {
    &chars[count..]
}

// 用于检测字符是关键字还是标识符
fn lookup_keyword(name: &str) -> TokenType {
    match name {
        "let" => TokenType::Let,
        "match" => TokenType::Match,
        "if" => TokenType::If,
        "then" => TokenType::Then,
        "else" => TokenType::Else,
        "for" => TokenType::For,
        "next" => TokenType::Next,
        "in" => TokenType::In,
        "branch" => TokenType::Branch,
        "each" => TokenType::Each,
        "mix" => TokenType::Mix,
        "which" => TokenType::Which,
        "where" => TokenType::Where,
        "only" => TokenType::Only,
        "within" => TokenType::Within,
        "into" => TokenType::Into,
        "regular" => TokenType::Regular,
        "template" => TokenType::Template,
        "to" => TokenType::To,
        "namespace" => TokenType::Namespace,
        "use" => TokenType::Use,
        "function" => TokenType::Function,
        "const" => TokenType::Const,
        "enum" => TokenType::Enum,
        "struct" => TokenType::Struct,
        "union" => TokenType::Union,
        "trait" => TokenType::Trait,
        "impl" => TokenType::Impl,
        "alias" => TokenType::Alias,

        // 返回标识符
        _ => TokenType::Identifier(name.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::tokenize;

    #[test]
    fn test_whitespace() {
        let tokens = tokenize(" \t").unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_comment() {
        let tokens1 = tokenize("/").unwrap();
        assert_eq!(tokens_to_string(&tokens1), vec!["/"]);

        let tokens2 = tokenize("/ // + - * /").unwrap();
        assert_eq!(tokens_to_string(&tokens2), vec!["/"]);
    }

    #[test]
    fn test_punctuation_marks() {
        let tokens1 = tokenize("{ } = >> | || && == != > >= < <= ++ + - * /").unwrap();
        assert_eq!(
            tokens_to_string(&tokens1),
            vec![
                "{", "}", "=", ">>", "|", "||", "&&", "==", "!=", ">", ">=", "<", "<=", "++", "+",
                "-", "*", "/",
            ]
        );

        let tokens2 = tokenize("?? & ^ ? . [ ] => ! ( ) # .. ... : ,").unwrap();
        assert_eq!(
            tokens_to_string(&tokens2),
            vec![
                "??", "&", "^", "?", ".", "[", "]", "=>", "!", "(", ")", "#", "..", "...", ":",
                ",",
            ]
        );
    }

    fn tokens_to_string(tokens: &[Token]) -> Vec<String> {
        let strings: Vec<String> = tokens.iter().map(|t| t.token_type.to_string()).collect();
        strings
    }
}
