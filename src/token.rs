/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use core::fmt;
use std::fmt::Write;

// 记录 Token 在源文件中的位置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file_id: usize, // 源文件 id
    pub start: usize,   // 开始位置
    pub end: usize,     // 结束位置（不包括）
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    NewLine,            // 换行符号，包括 '\r\n', '\n'， '\r' 以及 ';'
    Identifier(String), // 标识符

    // 字面量
    Integer(i64),           // 123, 1_001, 0xab, 0b1001
    Float(f64),             // 3.14, 1.6e-23,
    Imaginary(f64),         // 3i, 9.9i
    Bit(usize, Vec<u8>),    // 4'b1010, 8'xff, 8'd10
    Boolean(bool),          // true, false
    Char(char),             // 'a', '\x41', '\u{6587}'
    GeneralString(String),  // "foo"
    TemplateString(String), // `foo`
    HashString(String),     // #foo

    // 符号
    //
    // 符号名称参考
    // https://en.wikipedia.org/wiki/List_of_typographical_symbols_and_punctuation_marks
    //
    // 运算符优先级参考
    // https://en.wikipedia.org/wiki/Operators_in_C_and_C%2B%2B#Operator_precedence
    LeftBrace,  // {
    RightBrace, // }

    Assign,                // =
    Pipe,                  // |
    LogicOr,               // ||
    LogicAnd,              // &&
    Equal,                 // ==
    NotEqual,              // !=
    GreaterThan,           // >
    GreaterThanOrEqual,    // >=
    LessThan,              // <
    LessThanOrEqual,       // <=
    Forward,               // >>
    NamedOperator(String), // :name:
    Concat,                // ++
    Plus,                  // +
    Minus,                 // -
    Asterisk,              // *
    Slash,                 // /

    UnwrapOr, // ??
    Combine,  // &
    Cast,     // ^
    Unwrap,   // ?
    Dot,      // .

    LeftBracket,  // [
    RightBracket, // ]

    Arrow, // =>

    Exclamation, // !

    LeftParen,  // (
    RightParen, // )

    // 其他符号
    Hash,      // #
    Interval,  // ..
    Ellipsis,  // ...
    Separator, // ::
    Colon,     // :
    Comma,     // ,

    // 关键字
    Let,
    Do,
    Match,
    If,
    Then,
    Else,
    For,
    Next,
    In,
    Branch,
    Which,
    Where,
    Only,
    Into,
    Regular,
    Template,
    To,
    Namespace,
    Use,
    Function,
    Type,
    Const,
    Enum,
    Struct,
    Union,
    Trait,
    Impl,
    Alias,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenDetail {
    pub location: Location,
    pub token: Token,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "file id: {}, start: {}, end: {}",
            self.file_id, self.start, self.end
        )
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::NewLine => write!(f, "\n"),
            Token::Identifier(value) => write!(f, "{}", value),

            Token::Integer(value) => write!(f, "{}", value),
            Token::Float(value) => write!(f, "{}", value),
            Token::Imaginary(value) => write!(f, "{}i", value),
            Token::Bit(bit_width, bytes) => {
                let mut hex = String::new();
                for byte in bytes {
                    write!(hex, "{:02x}", byte)?;
                }
                write!(f, "{}'x{}", bit_width, hex)
            },
            Token::Boolean(value) => write!(f, "{}", value),
            Token::Char(value) => write!(f, "'{}'", value),
            Token::GeneralString(value) => write!(f, "\"{}\"", value),
            Token::TemplateString(value) => write!(f, "`{}`", value),
            Token::HashString(value) => write!(f, "#{}", value),

            Token::LeftBrace => write!(f, "{{"),  // {
            Token::RightBrace => write!(f, "}}"), // }

            Token::Assign => write!(f, "="),   // =
            Token::Pipe => write!(f, "|"),     // |
            Token::LogicOr => write!(f, "||"),    // ||
            Token::LogicAnd => write!(f, "&&"),   // &&
            Token::Equal => write!(f, "=="),      // ==
            Token::NotEqual => write!(f, "!="),   // !=
            Token::GreaterThan => write!(f, ">"), // >
            Token::GreaterThanOrEqual => write!(f, ">="), // >=
            Token::LessThan => write!(f, "<"),    // <
            Token::LessThanOrEqual => write!(f, "<="), // <=
            Token::Forward => write!(f, ">>"), // >>
            Token::NamedOperator(value) => write!(f, ":{}:", value),
            Token::Concat => write!(f, "++"), // ++
            Token::Plus /* Add */ => write!(f, "+"),      // +
            Token::Minus /* Subtract */ => write!(f, "-"), // -
            Token::Asterisk /* Multiply */ => write!(f, "*"), // *
            Token::Slash /* Divide */ => write!(f, "/"),   // /

            Token::UnwrapOr => write!(f, "??"), // ??
            Token::Combine => write!(f, "&"),   // &
            Token::Cast => write!(f, "^"), // ^
            Token::Unwrap => write!(f, "?"), // ?
            Token::Dot => write!(f, "."), // .

            Token::LeftBracket => write!(f, "["),  // [
            Token::RightBracket => write!(f, "]"), // ]

            Token::Arrow => write!(f, "=>"), // =>

            Token::Exclamation => write!(f, "!"), // !

            Token::LeftParen => write!(f, "("),  // (
            Token::RightParen => write!(f, ")"), // )

            // 其他符号
            Token::Hash => write!(f, "#"),       // #
            Token::Interval => write!(f, ".."),     // ..
            Token::Ellipsis => write!(f, "..."), // ...
            Token::Separator => write!(f, "::"), // ::
            Token::Colon => write!(f, ":"),      // :
            Token::Comma => write!(f, ","),      // ,

            // 关键字
            Token::Let => write!(f, "let"),
            Token::Do => write!(f, "do"),
            Token::Match => write!(f, "match"),
            Token::If => write!(f, "if"),
            Token::Then => write!(f, "then"),
            Token::Else => write!(f, "else"),
            Token::For => write!(f, "for"),
            Token::Next => write!(f, "next"),
            Token::In => write!(f, "in"),
            Token::Branch => write!(f, "branch"),
            Token::Which => write!(f, "which"),
            Token::Where => write!(f, "where"),
            Token::Only => write!(f, "only"),
            Token::Into => write!(f, "into"),
            Token::Regular => write!(f, "regular"),
            Token::Template => write!(f, "template"),
            Token::To => write!(f, "to"),
            Token::Namespace => write!(f, "namespace"),
            Token::Use => write!(f, "use"),
            Token::Function => write!(f, "function"),
            Token::Type => write!(f, "type"),
            Token::Const => write!(f, "const"),
            Token::Enum => write!(f, "enum"),
            Token::Struct => write!(f, "struct"),
            Token::Union => write!(f, "union"),
            Token::Trait => write!(f, "trait"),
            Token::Impl => write!(f, "impl"),
            Token::Alias => write!(f, "alias"),
        }
    }
}

impl fmt::Display for TokenDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.location, self.token)
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, Token, TokenDetail};

    #[test]
    fn test_location_display() {
        let ca1 = Location {
            file_id: 1,
            start: 2,
            end: 3,
        };

        assert_eq!(ca1.to_string(), "file id: 1, start: 2, end: 3");
    }

    #[test]
    fn test_location_eq() {
        let ca1 = Location {
            file_id: 1,
            start: 2,
            end: 3,
        };
        let ca2 = Location {
            file_id: 2,
            start: 2,
            end: 3,
        };
        let ca3 = Location {
            file_id: 1,
            start: 2,
            end: 3,
        };

        assert_ne!(ca1, ca2);
        assert_eq!(ca1, ca3);
    }

    #[test]
    fn test_token_display() {
        let tt1 = Token::Identifier("foo".to_string());
        let tt2 = Token::If;
        let tt3 = Token::Bit(8, vec![0xab, 0x4]);

        assert_eq!(tt1.to_string(), "foo");
        assert_eq!(tt2.to_string(), "if");
        assert_eq!(tt3.to_string(), "8'xab04")
    }

    #[test]
    fn test_token_eq() {
        let tt1 = Token::Identifier("foo".to_string());
        let tt2 = Token::GeneralString("foo".to_string());
        let tt3 = Token::Identifier("foo".to_string());

        assert_ne!(tt1, tt2);
        assert_eq!(tt1, tt3);

        assert_ne!(Token::Let, Token::For);
        assert_eq!(Token::Let, Token::Let);
    }

    #[test]
    fn test_token_detail_display() {
        let tk1 = TokenDetail {
            location: Location {
                file_id: 1,
                start: 2,
                end: 3,
            },
            token: Token::Plus, // Add,
        };

        let tk2 = TokenDetail {
            location: Location {
                file_id: 1,
                start: 2,
                end: 3,
            },
            token: Token::Minus, // Subtract,
        };

        assert_eq!(tk1.to_string(), "file id: 1, start: 2, end: 3 - +");
        assert_ne!(tk1, tk2);
    }
}
