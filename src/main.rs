use std::fs;
mod lexer;
mod parser;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    // let source = fs::read_to_string(&args[1]).expect("Couldn't read file '{}'");
    let source = "int main(void){\n\treturn 0;\n}";
    let mut lexer = Lexer::from_string(&source);

    let result = lexer.tokenise().unwrap();

    let mut parser = Parser::new(result);
    let ast = parser.parse()?;
    println!("{}", ast.print_tree());
    Ok(())
}
