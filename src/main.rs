mod lexer;
mod repl;
mod token;

use repl::Repl;

fn main() {
    let mut repl = Repl::new();
    repl.run();
}
