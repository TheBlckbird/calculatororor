use std::io::{stdin, stdout, Write};

use parser::parse;
use run::run;

mod ast;
mod parser;
mod run;
mod token;

fn main() {
    loop {
        print!("Calculation: ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let ast = match parse(input.as_str()) {
            Ok(ast) => ast,
            Err(_) => {
                println!("It seems like, you did something wrong :/\n");
                continue;
            }
        };
        let result = run(ast);

        println!("Your result is {result}\n");
    }
}
