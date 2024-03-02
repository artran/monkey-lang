#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    Illegal,
    EOF,
    // Ident(String),
    // Int(i32),
    Assign,
    Plus,
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // Function,
    // Let,
}
