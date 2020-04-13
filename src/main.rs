use std::env;

extern crate rcc1;
use rcc1::codegen::gen;
use rcc1::token::tokenize;
use rcc1::parse::expr;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
//    let s = args[1].clone();
    let s = "1>=0".to_string();
    let mut v = tokenize(s);

    let nodes = expr(&mut v);
    gen(nodes);

    println!("  pop rax");
    println!("  ret");
}
