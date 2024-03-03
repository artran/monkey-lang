#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Illegal,
    EOF,
    Ident(String),
    Int(i32),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
}
