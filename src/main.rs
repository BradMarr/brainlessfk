mod interactor;
use interactor::U8Extensions;

mod data_conversions;

fn main() {
    let mut pointer_index: u8 = 0;

    pointer_index.push_char('A', 6);
    pointer_index.print_char(6);
}