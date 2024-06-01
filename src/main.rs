mod interactor;

mod data_conversions;

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

    pointer.write_char('A');
    pointer.write_char('c');
    pointer.print_char(0);
    pointer.print_char(1);
    pointer.print_char(0);
}