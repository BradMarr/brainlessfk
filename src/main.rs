use std::{collections::HashMap, env};
use core::ops::Range;

mod interactor;
mod data_conversions;
mod string;
mod var;

mod parse_script;

#[derive(Debug)]
#[derive(Clone)]
pub enum StackValue {
    Char(char),
    Var(bool)
}

struct Pointer {
    index: u16,
    stack: Vec<StackValue>,
    occupied_index: u16,
    var_registry: HashMap::<String, Range<u16>>
}

fn main() {
    let mut pointer = Pointer {
        index: 0,
        stack: vec![StackValue::Char(0 as char); 30_000],
        occupied_index: 1,
        var_registry: HashMap::new(),
    };

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Please provide a script path");
    }
    let path = &args[1];

    pointer.exec_script(path);
}