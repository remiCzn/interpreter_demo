use crate::interpret::run;
use crate::parser::parse;
use std::fs;

mod ast;
mod errors;
mod interpret;
mod parser;
mod test;
mod typer;

fn main() {
    let arg1 = std::env::args().nth(1);
    let source = if let Some(path) = arg1 {
        fs::read_to_string(path).unwrap()
    } else {
        String::from("(4+3)*(6-1)")
    };
    println!("{:?}", run(source.as_str()));
}
