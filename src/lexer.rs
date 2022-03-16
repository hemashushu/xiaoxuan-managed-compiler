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
                            // lex comment
                            let post_rest = lex_comment(rest); // "//..."
                            post_rest
                        } else {
                            add_token(&mut tokens, new_token(TokenType::Slash)); // "/"
                            rest
                        }
                    }

                    // new line
                    '\n' | '\r' | ';' => {
                        let post_rest = lex_new_line(rest);
                        add_token(&mut tokens, new_token(TokenType::NewLine)); // "\n"
                        post_rest
                    }

                    // 符号
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

                    ',' => {
                        add_token(&mut tokens, new_token(TokenType::Comma)); // ","
                        rest
                    }

                    // 带符号的字面量
                    '\'' => {
                        // 字符 Char
                        let (token, post_rest) = lex_char(rest)?;
                        add_token(&mut tokens, token);
                        post_rest
                    }

                    '"' => {
                        // 字符串 String
                        let (token, post_rest) = lex_string(rest)?;
                        add_token(&mut tokens, token);
                        post_rest
                    }

                    '`' => {
                        // 模板字符串
                        let (token, post_rest) = lex_template_string(rest)?;
                        add_token(&mut tokens, token);
                        post_rest
                    }

                    '0' => {
                        if match_char('x', rest) {
                            // `十六进制` 整数
                            todo!()
                            // let (token, post_rest) = lex_16_radix_integer(rest)?;
                            // add_token(&mut tokens, token);
                            // post_rest
                        } else if match_char('b', rest) {
                            // `二进制` 整数
                            todo!()
                            // let (token, post_rest) = lex_2_radix_integer(rest)?;
                            // add_token(&mut tokens, token);
                            // post_rest
                        } else if match_char('.', rest) {
                            // `0.` 整数部分为 0 的浮点数
                            todo!()
                            // let (token, post_rest) = lex_zero_point_float(rest)?;
                            // add_token(&mut tokens, token);
                            // post_rest
                        } else {
                            match rest.first() {
                                Some(second_char) => {
                                    if is_letter(*second_char) {
                                        // 数字 0 开头的符号不是合法的标识符
                                        return Err(Error::LexerError("invalid identifier"));
                                    } else {
                                        // 普通整数 0
                                        add_token(&mut tokens, new_token(TokenType::Integer(0))); // "0"
                                        rest
                                    }
                                }
                                None => {
                                    // 普通整数 0
                                    add_token(&mut tokens, new_token(TokenType::Integer(0))); // "0"
                                    rest
                                }
                            }
                        }
                    }

                    '#' => {
                        match rest.first() {
                            Some(second_char) => {
                                if is_valid_first_letter_of_identifier(*second_char) {
                                    // 哈希字符串
                                    let (token, post_rest) = lex_hash_string(rest)?;
                                    add_token(&mut tokens, token);
                                    post_rest
                                } else {
                                    // 普通 # 符号
                                    add_token(&mut tokens, new_token(TokenType::Hash)); // "#"
                                    rest
                                }
                            }
                            None => {
                                add_token(&mut tokens, new_token(TokenType::Hash)); // "#"
                                rest
                            }
                        }
                    }

                    ':' => {
                        match rest.first() {
                            Some(second_char) => {
                                if *second_char == ':' {
                                    // 命名空间路径的分隔符
                                    add_token(&mut tokens, new_token(TokenType::Separator)); // "::"
                                    move_forword(rest, 1)
                                } else if is_valid_first_letter_of_identifier(*second_char) {
                                    match lex_named_operator(rest) {
                                        Ok((token, post_rest)) => {
                                            // 解析为命名操作符
                                            add_token(&mut tokens, token);
                                            post_rest
                                        }
                                        Err(_) => {
                                            // 解析为命名操作符失败，将 `:` 符号作为普通的冒号
                                            add_token(&mut tokens, new_token(TokenType::Colon)); // ":"
                                            rest
                                        }
                                    }
                                } else {
                                    // 普通的冒号
                                    add_token(&mut tokens, new_token(TokenType::Colon)); // ":"
                                    rest
                                }
                            }
                            None => {
                                // 普通的冒号
                                add_token(&mut tokens, new_token(TokenType::Colon)); // ":"
                                rest
                            }
                        }
                    }

                    // 整数、浮点数、比特数、标识符、关键字等
                    _ => {
                        if is_none_zero_number(*first) {
                            // 整数、浮点数或者比特数
                            let (token, post_rest) = lex_number(chars)?;
                            add_token(&mut tokens, token);
                            post_rest
                        } else if is_valid_first_letter_of_identifier(*first) {
                            // 标识符或者关键字
                            let (token, post_rest) = lex_identifier_or_keyword(chars)?;
                            add_token(&mut tokens, token);
                            post_rest
                        } else {
                            return Err(Error::LexerError("unexpected char"));
                        }
                    }
                };
            }
            None => break,
        };
    }

    Ok(tokens)
}

fn lex_comment(source_chars: &[char]) -> &[char] {
    // 行注释，跳过所有字符直到行尾（'\n' 或者 '\r\n'）
    // 注意要保留换行符到返回的字符数组（rest）中
    match source_chars.iter().position(|c| *c == '\n') {
        Some(index) => &source_chars[index..],
        None => &source_chars[source_chars.len()..],
    }
}

fn lex_new_line(source_chars: &[char]) -> &[char] {
    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => match *first {
                '\n' | '\r' | ';' => {
                    // ';' 符号视为换行符
                    end_pos += 1;
                    rest
                }
                ' ' | '\t' => {
                    // 跳过行与行之间的空白
                    end_pos += 1;
                    rest
                }
                _ => {
                    break;
                }
            },
            None => {
                break;
            }
        }
    }

    move_forword(source_chars, end_pos)
}

fn lex_char(source_chars: &[char]) -> Result<(Token, &[char]), Error> {
    // 查找 `字符字面量` 的结束字符 `'`，但不包括 `\'`
    // e.g.
    // 'a'
    // '\n'
    // '\xHH'
    // '\u{H}' ~ '\u{HHHHHH}'
    // '\omega'
    //  ^-------- 当前所在的位置

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '\\' => {
                        if match_char('\'', rest) {
                            // 找到了 '\''
                            end_pos += 2;
                            move_forword(rest, 1)
                        } else {
                            // 找到了其他转义字符
                            // todo::
                            end_pos += 1;
                            rest
                        }
                    }
                    '\'' => {
                        break;
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // 到了末尾仍未找到结束字符
                return Err(Error::LexerError("expected char literal ending mark"));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];

    // todo:: 处理转义字符
    // todo:: 验证字符的有效性

    // 当前 end_pos 处于字符 `'` 位置
    // 剩余的字符应该从 `'` 位置之后开始

    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token(TokenType::Char(value_chars[0])), rest))
}

fn lex_string(source_chars: &[char]) -> Result<(Token, &[char]), Error> {
    // 查找 `字符串字面量` 的结束字符 `"`，但不包括 `\"`
    // e.g.
    // "foo bar"
    //  ^-------- 当前所在的位置

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '\\' => {
                        if match_char('"', rest) {
                            // 找到了 '"'
                            end_pos += 2;
                            move_forword(rest, 1)
                        } else {
                            // 找到了其他转义字符
                            end_pos += 1;
                            rest
                        }
                    }
                    '\"' => {
                        break;
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // 到了末尾仍未找到结束字符
                return Err(Error::LexerError("expected string literal ending mark"));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // todo:: 处理转义字符

    // 当前 end_pos 处于字符 `"` 位置
    // 剩余的字符应该从 `"` 位置之后开始
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token(TokenType::String(value)), rest))
}

fn lex_template_string(source_chars: &[char]) -> Result<(Token, &[char]), Error> {
    // 查找 `模板字符串字面量` 的结束字符 '`'，但不包括 '`'
    // e.g.
    // `foo bar`
    //  ^-------- 当前所在的位置

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    loop {
        match chars.split_first() {
            Some((first, rest)) => {
                chars = match *first {
                    '\\' => {
                        if match_char('`', rest) {
                            // 找到了 '`'
                            end_pos += 2;
                            move_forword(rest, 1)
                        } else {
                            // 找到了其他转义字符
                            end_pos += 1;
                            rest
                        }
                    }
                    '`' => {
                        break;
                    }
                    _ => {
                        end_pos += 1;
                        rest
                    }
                }
            }
            None => {
                // 到了末尾仍未找到结束字符
                return Err(Error::LexerError(
                    "expected template string literal ending mark",
                ));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // todo:: 处理转义字符

    // 当前 end_pos 处于字符 '`' 位置
    // 剩余的字符应该从 '`' 位置之后开始
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token(TokenType::TemplateString(value)), rest))
}

fn lex_hash_string(source_chars: &[char]) -> Result<(Token, &[char]), Error> {
    // 查找连续的字符
    // e.g.
    // #foo_bar
    //  ^-------- 当前所在的位置

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // 注：
    // 第一个字符已经验证过是合法的标识符首个字符

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) if is_letter(*first) => {
                end_pos += 1;
                rest
            }
            Some(_) => {
                break;
            }
            None => {
                break;
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // 当前 end_pos 处于标识符的最后一个字符位置
    // 剩余的字符应该从标识符位置之后开始，即跳过 end_pos 个字符即可。
    let rest = move_forword(source_chars, end_pos);
    Ok((new_token(TokenType::HashString(value)), rest))
}

fn lex_named_operator(source_chars: &[char]) -> Result<(Token, &[char]), Error> {
    // 查找连续的字符，以及结束的 `:` 符号
    // e.g.
    // :foo_bar:
    //  ^-------- 当前所在的位置

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // 注：
    // 第一个字符已经验证过是合法的标识符首个字符

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                if *first == ':' {
                    // 已找到结束符
                    break;
                } else if is_letter(*first) {
                    // 仍在有效标识符字符之中
                    end_pos += 1;
                    rest
                } else {
                    // 遇到无效的标识符字符
                    return Err(Error::LexerError("invalid identifier letter"));
                }
            }
            None => {
                // 到了末尾仍未找到结束字符
                return Err(Error::LexerError("expected named operator ending mark"));
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // 当前 end_pos 处于字符 `:` 位置
    // 剩余的字符应该从 `:` 位置之后开始
    let rest = move_forword(source_chars, end_pos + 1);
    Ok((new_token(TokenType::NamedOperator(value)), rest))
}

fn lex_number(source_chars: &[char]) -> Result<(Token, &[char]), Error> {
    // 整数、浮点数或者比特数
    //
    // 查找连续的数字
    // e.g.
    // 123
    // 1_234
    // 3i
    // 9.9i
    // 8'xff
    // 4'b01_10
    // 2.71828
    // 6.626e-34
    // ^-------- 当前所在的位置

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // 注：
    // 第一个字符已经验证过是合法的标识符首个数字

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                match *first {
                    '0'..='9' | '_' => {
                        // 仍在有效的数字之中
                        end_pos += 1;
                        rest
                    }
                    '.' => {
                        return continue_lex_float_number(&source_chars[..end_pos], rest);
                    }
                    '\'' => {
                        return continue_lex_bit_number(&source_chars[..end_pos], rest);
                    }
                    'i' => {
                        return continue_lex_imaginary_number(&source_chars[..end_pos], rest);
                    }
                    'e' => {
                        return continue_lex_float_number_exponent(&source_chars[..end_pos], rest);
                    }
                    _ => {
                        // 遇到了一个非数字
                        break;
                    }
                }
            }
            None => {
                // 到了末尾
                break;
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value_string = value_chars
        .iter()
        .filter(|c| **c != '_') // 移除字符串当中的下划线
        .collect::<String>();

    // 将字符串转换为数字
    let value: i64 = value_string
        .parse()
        .map_err(|_| Error::LexerError("invalid integer number"))?;

    // 当前 end_pos 处于标识符的最后一个字符位置
    // 剩余的字符应该从标识符位置之后开始，即跳过 end_pos 个字符即可。
    let rest = move_forword(source_chars, end_pos);

    Ok((new_token(TokenType::Integer(value)), rest))
}

fn continue_lex_float_number<'a>(
    previous_chars: &'a [char],
    remain_chars: &'a [char],
) -> Result<(Token, &'a [char]), Error> {
    todo!()
}

fn continue_lex_imaginary_number<'a>(
    previous_chars: &'a [char],
    remain_chars: &'a [char],
) -> Result<(Token, &'a [char]), Error> {
    todo!()
}

fn continue_lex_bit_number<'a>(
    previous_chars: &'a [char],
    remain_chars: &'a [char],
) -> Result<(Token, &'a [char]), Error> {
    todo!()
}

fn continue_lex_float_number_exponent<'a>(
    previous_chars: &'a [char],
    remain_chars: &'a [char],
) -> Result<(Token, &'a [char]), Error> {
    todo!()
}

fn lex_identifier_or_keyword(source_chars: &[char]) -> Result<(Token, &[char]), Error> {
    // 标识符或者关键字
    //
    // 查找连续的字符
    // e.g.
    // foo_bar
    // ^-------- 当前所在的位置

    let mut chars = source_chars;
    let mut end_pos: usize = 0;

    // 注：
    // 第一个字符已经验证过是合法的标识符首个字符

    loop {
        chars = match chars.split_first() {
            Some((first, rest)) => {
                if is_letter(*first) {
                    // 仍在有效标识符字符之中
                    end_pos += 1;
                    rest
                } else {
                    // 遇到无效的标识符字符，提前退出循环
                    break;
                }
            }
            None => {
                // 到了末尾
                break;
            }
        }
    }

    let value_chars = &source_chars[..end_pos];
    let value = value_chars.iter().collect::<String>();

    // 当前 end_pos 处于标识符的最后一个字符位置
    // 剩余的字符应该从标识符位置之后开始，即跳过 end_pos 个字符即可。
    let rest = move_forword(source_chars, end_pos);

    match lookup_keyword(&value) {
        Some(token_type) => Ok((new_token(token_type), rest)),
        None => Ok((new_token(TokenType::Identifier(value)), rest)),
    }
}

fn is_none_zero_number(c: char) -> bool {
    match c {
        '1'..='9' => true,
        _ => false,
    }
}

// 可以作为标识符第一个文字的文字
fn is_valid_first_letter_of_identifier(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        _ => false,
    }
}

// 所有合法的文字（数字、字母、中文文字等）
fn is_letter(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => true,
        _ => false,
    }
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

fn move_forword(chars: &[char], count: usize) -> &[char] {
    &chars[count..]
}

// 用于检测字符是关键字还是标识符
fn lookup_keyword(name: &str) -> Option<TokenType> {
    match name {
        "let" => Some(TokenType::Let),
        "match" => Some(TokenType::Match),
        "if" => Some(TokenType::If),
        "then" => Some(TokenType::Then),
        "else" => Some(TokenType::Else),
        "for" => Some(TokenType::For),
        "next" => Some(TokenType::Next),
        "in" => Some(TokenType::In),
        "branch" => Some(TokenType::Branch),
        "each" => Some(TokenType::Each),
        "mix" => Some(TokenType::Mix),
        "which" => Some(TokenType::Which),
        "where" => Some(TokenType::Where),
        "only" => Some(TokenType::Only),
        "within" => Some(TokenType::Within),
        "into" => Some(TokenType::Into),
        "regular" => Some(TokenType::Regular),
        "template" => Some(TokenType::Template),
        "to" => Some(TokenType::To),
        "namespace" => Some(TokenType::Namespace),
        "use" => Some(TokenType::Use),
        "function" => Some(TokenType::Function),
        "const" => Some(TokenType::Const),
        "enum" => Some(TokenType::Enum),
        "struct" => Some(TokenType::Struct),
        "union" => Some(TokenType::Union),
        "trait" => Some(TokenType::Trait),
        "impl" => Some(TokenType::Impl),
        "alias" => Some(TokenType::Alias),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::token::Token;

    use super::tokenize;

    fn tokens_to_string(tokens: &[Token]) -> Vec<String> {
        let strings: Vec<String> = tokens.iter().map(|t| t.token_type.to_string()).collect();
        strings
    }

    #[test]
    fn test_whitespace() {
        let tokens = tokenize(" \t").unwrap();
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_comment() {
        let tokens1 = tokenize("/").unwrap();
        assert_eq!(tokens_to_string(&tokens1), vec!["/"]);

        let tokens2 = tokenize("/ // comment").unwrap();
        assert_eq!(tokens_to_string(&tokens2), vec!["/"]);

        let tokens3 = tokenize("/ // comment\n/").unwrap();
        assert_eq!(tokens_to_string(&tokens3), vec!["/", "\n", "/"]);
    }

    #[test]
    fn test_new_line() {
        let tokens1 = tokenize("\n \r\n").unwrap();
        assert_eq!(tokens_to_string(&tokens1), vec!["\n"]);

        let tokens2 = tokenize("; \n").unwrap();
        assert_eq!(tokens_to_string(&tokens2), vec!["\n"]);
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

        let tokens2 = tokenize("?? & ^ ? . [ ] => ! ( ) # .. ... ,").unwrap();
        assert_eq!(
            tokens_to_string(&tokens2),
            vec!["??", "&", "^", "?", ".", "[", "]", "=>", "!", "(", ")", "#", "..", "...", ",",]
        );
    }

    #[test]
    fn test_char_literal() {
        let tokens1 = tokenize("'a' 'b'").unwrap();
        assert_eq!(tokens_to_string(&tokens1), vec!["'a'", "'b'"]);
        // todo:: 测试转义字符
    }

    #[test]
    fn test_string_literal() {
        let tokens1 = tokenize(r#""foo" "b\"ar\"" "y&x""#).unwrap();
        assert_eq!(
            tokens_to_string(&tokens1),
            vec!["\"foo\"", "\"b\\\"ar\\\"\"", "\"y&x\""]
        );
        // todo:: 测试转义字符
    }

    #[test]
    fn test_template_string_literal() {
        let tokens1 = tokenize(r#" `foo` `b'a"r` "#).unwrap();
        assert_eq!(tokens_to_string(&tokens1), vec![r#"`foo`"#, r#"`b'a"r`"#]);
        // todo:: 测试转义字符
    }

    #[test]
    fn test_hash() {
        // 测试 `井号` 以及 `哈希字符串`，比如 `#`, `#foo`
        let tokens1 = tokenize("# #foo #bar").unwrap();
        assert_eq!(tokens_to_string(&tokens1), vec!["#", "#foo", "#bar"]);
    }

    #[test]
    fn test_colon() {
        // 测试 `冒号`、`命名空间路径分隔符`、`命名操作符`，比如 `:`，`::`，`:foo: :bar:`
        let tokens1 = tokenize(": :: :\"value\" :foo: :bar:").unwrap();
        assert_eq!(
            tokens_to_string(&tokens1),
            vec![":", "::", ":", "\"value\"", ":foo:", ":bar:"]
        );
    }

    #[test]
    fn test_integer_number() {
        let tokens1 = tokenize("1 100 1_234 1_2_3").unwrap();
        assert_eq!(tokens_to_string(&tokens1), vec!["1", "100", "1234", "123"]);
    }

    #[test]
    fn test_identifier() {
        let tokens1 = tokenize("a ab a_b a123 _ _a a_").unwrap();
        assert_eq!(
            tokens_to_string(&tokens1),
            vec!["a", "ab", "a_b", "a123", "_", "_a", "a_"]
        );
    }

    #[test]
    fn test_keywords() {
        let tokens1 = tokenize("let match if then else trait").unwrap();
        assert_eq!(
            tokens_to_string(&tokens1),
            vec!["let", "match", "if", "then", "else", "trait"]
        );
    }
}
