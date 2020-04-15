use std::env;

extern crate rcc1;
use rcc1::codegen::gen;
use rcc1::token::tokenize;
use rcc1::parse::program;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");

    println!("  push rbp");
    println!("  mov rbp, rsp");
    println!("  sub rsp, 208");
    let s = args[1].clone();
//    let s = "a=3+1;b=4+a;b+1;".to_string();
    let mut v = tokenize(s);

    let nodes = program(&mut v);
    for i in nodes{

        gen(i);
        println!("  pop rax");
    }

    println!("  mov rsp, rbp");
    println!("  pop rbp");
    println!("  ret");
}
