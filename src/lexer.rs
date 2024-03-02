use crate::token::Token;

struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    fn new(input: String) -> Lexer {
        Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn next_token(&mut self) -> Token {
        self.read_char();

        match self.ch {
            '=' => Token::Assign,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ',' => Token::Comma,
            '+' => Token::Plus,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '\0' => Token::EOF,
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
        r#"=+(){},;"#.to_string()
    }

    #[fixture]
    fn expected() -> Vec<Token> {
        vec![
            Token::Assign,
            Token::Plus,
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::RBrace,
            Token::Comma,
            Token::Semicolon,
            Token::EOF,
        ]
    }

    #[rstest]
    fn test_next_token(input: String, expected: Vec<Token>) {
        let mut l = Lexer::new(input.to_string());

        for test in expected {
            let tok = l.next_token();
            assert_that!(tok).is_equal_to(test);
        }
    }
}
