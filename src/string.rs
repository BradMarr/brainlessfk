use crate::Pointer;
use std::ops::Range;

impl Pointer {
    pub fn write_str(&mut self, str: &str) {
        for char in str.chars() {
            self.write_char(char);
        }
    }
    pub fn print_str(&mut self, range: Range<u8>) {
        for index in range {
            self.print_char(index);
        }
    }
}