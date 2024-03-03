use crate::token::Token;

pub(crate) struct Lexer {
    input: String,
    next_position: usize,
    current_char: char,
}

impl Lexer {
    pub(crate) fn new(input: String) -> Lexer {
        let current_char = &input.chars().nth(0).unwrap();
        Lexer {
            input,
            next_position: 0,
            current_char: *current_char,
        }
    }

    fn read_char(&mut self) {
        if self.next_position >= self.input.len() {
            self.current_char = '\0';
        } else {
            self.current_char = self.input.chars().nth(self.next_position).unwrap();
        }
        self.next_position += 1;
        dbg!(&self.current_char);
    }

    /// Push back the current character to the input
    fn push_back(&mut self) {
        self.next_position -= 1;
        self.current_char = self.input.chars().nth(self.next_position).unwrap();
    }

    fn read_consecutive_chars(&mut self, predicate: fn(char) -> bool) -> String {
        // We need to go back one position to get the start of the token
        let start = self.next_position - 1;
        while predicate(self.current_char) {
            self.read_char();
        }
        // We've gone one char too far, so we need to push back
        self.push_back();
        let end = self.next_position;
        self.input[start..end].to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.is_ascii_whitespace() {
            dbg!("skip_whitespace");
            self.read_char();
        }
    }

    pub(crate) fn next_token(&mut self) -> Token {
        self.read_char();
        self.skip_whitespace();

        match self.current_char {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\0' => Token::EOF,
            '0'..='9' => {
                let number = self
                    .read_consecutive_chars(|c| c.is_ascii_digit())
                    .parse::<i32>()
                    .unwrap();
                Token::Int(number)
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_consecutive_chars(|c| c.is_ascii_alphanumeric() || c == '_');
                match ident.as_str() {
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    _ => Token::Ident(ident),
                }
            }
            _ => Token::Illegal,
        }
    }
}

/******************************************************************************
* Tests
******************************************************************************/
#[cfg(test)]
mod exoected {
    use super::*;
    use rstest::*;
    use speculoos::prelude::*;

    #[fixture]
    fn input() -> String {
        r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "#
        .to_string()
    }

    #[fixture]
    fn expected() -> Vec<Token> {
        vec![
            Token::Let,
            Token::Ident("five".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Let,
            Token::Ident("ten".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Let,
            Token::Ident("add".to_string()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".to_string()),
            Token::Comma,
            Token::Ident("y".to_string()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".to_string()),
            Token::Plus,
            Token::Ident("y".to_string()),
            Token::Semicolon,
            Token::RBrace,
            Token::Semicolon,
            Token::Let,
            Token::Ident("result".to_string()),
            Token::Assign,
            Token::Ident("add".to_string()),
            Token::LParen,
            Token::Ident("five".to_string()),
            Token::Comma,
            Token::Ident("ten".to_string()),
            Token::RParen,
            Token::Semicolon,
            Token::EOF,
        ]
    }

    #[rstest]
    fn test_next_token(input: String, expected: Vec<Token>) {
        let mut l = Lexer::new(input.to_string());

        for test in expected {
            let tok = l.next_token();
            dbg!(&tok);
            assert_that!(tok).is_equal_to(test);
        }
    }
}
