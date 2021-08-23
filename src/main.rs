use logos::Lexer as LogosLexer;

use crate::parser::Lexer;

mod ast;
mod parser;

fn main() {
    let mut lexer = Lexer::new(LogosLexer::new("*\n\n*"));
    
    while let Some(token) = lexer.next_token() {
        println!("{:?}", token);
        println!("{:?}", lexer.position());
    }
}
