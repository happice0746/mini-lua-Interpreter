mod core;
use core::{parser, vm::ExeState};
use std::{env, fs::File};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: {} script", args[0]);
        return
    }
    let file = File::open(&args[1]).unwrap();
    let proto = parser::ParseProto::load(file);
    let mut exe = ExeState::new();
    exe.execute(&proto);
}
