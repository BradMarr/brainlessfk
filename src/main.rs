mod interactor;
mod data_conversions;
mod string;

struct Pointer {
    index: u8,
    stack: Vec<char>,
    stack_index: u8,
}

fn main() {
    let mut pointer = Pointer {
        index: 0,
        stack: vec![0 as char; 30_000],
        stack_index: 0,
    };
    
    pointer.write_str("Hello, World!");
    pointer.print_str(0..13);
}