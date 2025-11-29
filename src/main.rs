use std::fs;
mod lexer;
mod parser;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() <= 1 {
        println!("fcc help TODO");
        return ();
    }

    let source = fs::read_to_string(&args[1]).expect("Couldn't read file '{}'");
    let mut lexer = Lexer::from_string(&source);

    let result = lexer.tokenise().unwrap();

    let mut parser = Parser::new(result);
    parser.parse();
}
