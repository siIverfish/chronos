#![feature(box_patterns)]

mod interpreter;
mod ast;
mod parser;

use crate::interpreter::interpret;
// use crate::ast::*;

fn main() {
    // let f = Box::new(AST::Data(Data::Function(chronos_print)));
    // let arg = Box::new(AST::Data(Data::String("Hello, World!".to_string())));
    // interpret(AST::Application { f, arg });
    use parser::parse_chronos;
    let input = std::fs::read_to_string("./src/test.chr").unwrap();
    let code = parse_chronos(&input).unwrap();
    interpret(code);
}
