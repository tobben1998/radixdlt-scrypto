use lalrpop_util::lalrpop_mod;
use serde_json::to_string;
use std::env;
use std::fs;

pub mod ast;

lalrpop_mod!(pub rtd);

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let ast = rtd::TransactionParser::new().parse(&contents).unwrap();
    println!("{}", to_string(&ast).unwrap());
}
