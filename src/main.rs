mod lexer;
mod token;

use lexer::Lexer;

fn main() {
    let input = r#"
        let five = 5;
        let ten = 10;
        let add = fn(x, y) {
            x + y;
        };
        let result = add(five, ten);
        "#;

    let mut l = Lexer::new(input.to_string());
    println!("{:?}", l.next_token());
}
