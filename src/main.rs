use crate::interpret::run;
use crate::parser::parse;
use std::{
    fs,
    io::{self, Write},
};

mod ast;
mod errors;
mod interpret;
mod parser;
mod test;

fn main() {
    let arg1 = std::env::args().nth(1);
    let source = if let Some(path) = arg1 {
        fs::read_to_string(path).unwrap()
    } else {
        print!("MyLang 0.1.0 - Interpreter, just for fun -  by remiCzn \n>> ");
        io::stdout().flush().unwrap();
        let mut val = String::new();
        let stdin = io::stdin();
        while stdin.read_line(&mut val).unwrap() > 0 {
            //TODO: Interpret val here
            print!("{}\n>> ", val.trim());
            io::stdout().flush().unwrap();
            val.clear();
        }
        return;
    };
    println!("{:?}", run(source.as_str()));
}
