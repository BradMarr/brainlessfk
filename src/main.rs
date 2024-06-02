use std::collections::HashMap;
use core::ops::Range;

mod interactor;
mod data_conversions;
mod string;
mod var;

mod parse_script;
use crate::parse_script::StringExtensions;
use crate::parse_script::VecStrExtensions;

struct Pointer {
    index: u16,
    stack: Vec<char>,
    occupied_index: u16,
    var_registry: HashMap::<String, Range<u16>>
}

fn main() {
    let mut pointer = Pointer {
        index: 0,
        stack: vec![0 as char; 30_000],
        occupied_index: 1,
        var_registry: HashMap::new(),
    };

    for mut line in pointer.split_to_line_vector("./scripts/test.blf") {
        line.split_args().exec_command(&mut pointer);
    }
}