extern crate core;

use crate::interpret::interpret;
use crate::parser::parse;
use std::fs;

mod ast;
mod interpret;
mod parser;
mod test;

fn main() {
    let arg1 = std::env::args().nth(1);
    let source = if let Some(path) = arg1 {
        let str = fs::read_to_string(path).unwrap();
        str
    } else {
        String::from("(4+3)*(6-1)")
    };
    let ast = parse(source.as_str()).unwrap();
    println!("AST: {:?}", ast);
    println!("{:?}", interpret(ast));
}
