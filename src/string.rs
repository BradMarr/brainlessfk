use crate::Pointer;
use std::ops::Range;

impl Pointer {
    /// Pushes @str to stack.
    pub fn write_str(&mut self, str: &str) {
        for char in str.chars() {
            self.write_char(char);
        }
    }
    /// Prints all chars in @range from stack.
    pub fn print_str_from_stack(&mut self, range: Range<u16>) {
        for index in range {
            self.print_char_from_stack(index);
        }
    }
    /// Prints @value (streams through buffer).
    pub fn print_str(&mut self, value: &str) {
        for char in value.chars() {
            self.push_char(char, 0);
            self.print_char_from_stack(0);
        }
    }
}