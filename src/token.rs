/**
 * Copyright (c) 2022 Hemashushu <hippospark@gmail.com>, All rights reserved.
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */
use core::fmt;

// token 的源文件id， 开始位置，结束位置
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Location {
    pub file_id: usize,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    NewLine, // 换行 '\r\n', '\n', 包括 ';'，不考虑单独一个 '\r' 字符的换行符的情况

    // 标识符
    Identifier(String),

    // 字面量
    Integer(i64),   // 123, 1_001, 0xab, 0b1001
    Float(f64),     // 3.14, 1.6e-23,
    Imaginary(f64), // 3i, 9.9i
    Bit(u8, i64),   // 4'b1010, 8'xff, 8'd10

    Boolean(bool),
    Char(char),
    String(String),
    TemplateString(String),
    HashString(String),
    // Regexp(String),

    // 符号名称参考
    // https://en.wikipedia.org/wiki/List_of_typographical_symbols_and_punctuation_marks

    // 符号
    LeftBrace,  // {
    RightBrace, // }

    Assign,  // =
    Forward, // >>
    Pipe,    // |

    NamedOperator(String), // :name:
    LogicOr,               // ||
    LogicAnd,              // &&
    Equal,                 // ==
    NotEqual,              // !=
    GreaterThan,           // >
    GreaterThanOrEqual,    // >=
    LessThan,              // <
    LessThanOrEqual,       // <=

    Concat, // ++

    Plus,     // Add,      // +
    Minus,    // Subtract, // -
    Asterisk, // Multiply, // *
    Slash,    // Divide,   // /

    UnwrapOr, // ??
    Combine,  // &

    Cast,   // ^ type casting/convert
    Unwrap, // ?

    Dot, // .

    LeftBracket,  // [
    RightBracket, // ]

    Arrow, // =>

    Exclamation, // !

    LeftParen,  // (
    RightParen, // )

    // 其他符号
    Hash,      // #
    Range,     // ..
    Ellipsis,  // ...
    Separator, // ::
    Colon,     // :
    Comma,     // ,

    // 关键字
    Let,
    Match,
    If,
    Then,
    Else,
    For,
    Next,
    In,
    Branch,
    Each,
    Mix,
    Which,
    Where,
    Only,
    Within,
    Into,
    Regular,
    Template,
    To,
    Namespace,
    Use,
    Function,
    Const,
    Enum,
    Struct,
    Union,
    Trait,
    Impl,
    Alias,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    // pub raw: String,
    pub location: Location,
    pub token_type: TokenType,
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

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // TokenType::EOF => write!(f, "EOF"),
            TokenType::NewLine => write!(f, "\n"),

            TokenType::Identifier(value) => write!(f, "{}", value),

            TokenType::Integer(value) => write!(f, "{}", value),
            TokenType::Float(value) => write!(f, "{}", value),
            TokenType::Bit(bit_width, value) => write!(f, "{}d{}", bit_width, value),
            TokenType::Imaginary(value) => write!(f, "{}i", value),
            TokenType::Boolean(value) => write!(f, "{}", value),
            TokenType::Char(value) => write!(f, "'{}'", value),
            TokenType::String(value) => write!(f, "\"{}\"", value),
            TokenType::TemplateString(value) => write!(f, "`{}`", value),
            TokenType::HashString(value) => write!(f, "#{}", value),
            // TokenType::Regexp(value) => write!(f, "{}", value),

            TokenType::LeftBrace => write!(f, "{{"),  // {
            TokenType::RightBrace => write!(f, "}}"), // }

            TokenType::Assign => write!(f, "="),   // =
            TokenType::Forward => write!(f, ">>"), // >>
            TokenType::Pipe => write!(f, "|"),     // |

            TokenType::NamedOperator(value) => write!(f, ":{}:", value),
            TokenType::LogicOr => write!(f, "||"),    // ||
            TokenType::LogicAnd => write!(f, "&&"),   // &&
            TokenType::Equal => write!(f, "=="),      // ==
            TokenType::NotEqual => write!(f, "!="),   // !=
            TokenType::GreaterThan => write!(f, ">"), // >
            TokenType::GreaterThanOrEqual => write!(f, ">="), // >=
            TokenType::LessThan => write!(f, "<"),    // <
            TokenType::LessThanOrEqual => write!(f, "<="), // <=

            TokenType::Concat => write!(f, "++"), // ++

            TokenType::Plus /* Add */ => write!(f, "+"),      // +
            TokenType::Minus /* Subtract */ => write!(f, "-"), // -
            TokenType::Asterisk /* Multiply */ => write!(f, "*"), // *
            TokenType::Slash /* Divide */ => write!(f, "/"),   // /

            TokenType::UnwrapOr => write!(f, "??"), // ??
            TokenType::Combine => write!(f, "&"),   // &

            TokenType::Cast => write!(f, "^"), // ^ type casting/convert
            TokenType::Unwrap => write!(f, "?"), // ?

            TokenType::Dot => write!(f, "."), // .

            TokenType::LeftBracket => write!(f, "["),  // [
            TokenType::RightBracket => write!(f, "]"), // ]

            TokenType::Arrow => write!(f, "=>"), // =>

            TokenType::Exclamation => write!(f, "!"), // !

            TokenType::LeftParen => write!(f, "("),  // (
            TokenType::RightParen => write!(f, ")"), // )

            // 其他符号
            TokenType::Hash => write!(f, "#"),       // #
            TokenType::Range => write!(f, ".."),     // ..
            TokenType::Ellipsis => write!(f, "..."), // ...
            TokenType::Separator => write!(f, "::"), // ::
            TokenType::Colon => write!(f, ":"),      // :
            TokenType::Comma => write!(f, ","),      // ,

            // 关键字
            TokenType::Let => write!(f, "let"),
            TokenType::Match => write!(f, "match"),
            TokenType::If => write!(f, "if"),
            TokenType::Then => write!(f, "then"),
            TokenType::Else => write!(f, "else"),
            TokenType::For => write!(f, "for"),
            TokenType::Next => write!(f, "next"),
            TokenType::In => write!(f, "in"),
            TokenType::Branch => write!(f, "branch"),
            TokenType::Each => write!(f, "each"),
            TokenType::Mix => write!(f, "mix"),
            TokenType::Which => write!(f, "which"),
            TokenType::Where => write!(f, "where"),
            TokenType::Only => write!(f, "only"),
            TokenType::Within => write!(f, "within"),
            TokenType::Into => write!(f, "into"),
            TokenType::Regular => write!(f, "regular"),
            TokenType::Template => write!(f, "template"),
            TokenType::To => write!(f, "to"),
            TokenType::Namespace => write!(f, "namespace"),
            TokenType::Use => write!(f, "use"),
            TokenType::Function => write!(f, "function"),
            TokenType::Const => write!(f, "const"),
            TokenType::Enum => write!(f, "enum"),
            TokenType::Struct => write!(f, "struct"),
            TokenType::Union => write!(f, "union"),
            TokenType::Trait => write!(f, "trait"),
            TokenType::Impl => write!(f, "impl"),
            TokenType::Alias => write!(f, "alias"),

        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.location, self.token_type)
    }
}

#[cfg(test)]
mod tests {
    use super::{Location, Token, TokenType};

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
    fn test_token_type_display() {
        let tt1 = TokenType::Identifier("foo".to_string());
        let tt2 = TokenType::If;

        assert_eq!(tt1.to_string(), "foo");
        assert_eq!(tt2.to_string(), "if");
    }

    #[test]
    fn test_token_type_eq() {
        let tt1 = TokenType::Identifier("foo".to_string());
        let tt2 = TokenType::String("foo".to_string());
        let tt3 = TokenType::Identifier("foo".to_string());

        assert_ne!(tt1, tt2);
        assert_eq!(tt1, tt3);

        assert_ne!(TokenType::Let, TokenType::For);
        assert_eq!(TokenType::Let, TokenType::Let);
    }

    #[test]
    fn test_token_display() {
        let tk1 = Token {
            location: Location {
                file_id: 1,
                start: 2,
                end: 3,
            },
            token_type: TokenType::Plus, // Add,
        };

        let tk2 = Token {
            location: Location {
                file_id: 1,
                start: 2,
                end: 3,
            },
            token_type: TokenType::Minus, // Subtract,
        };

        assert_eq!(tk1.to_string(), "file id: 1, start: 2, end: 3 - +");
        assert_ne!(tk1, tk2);
    }
}
