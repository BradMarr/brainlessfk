use crate::Pointer;

trait I8Extensions {
    fn add_to_cell(&self);
}

impl I8Extensions for i8 {
    fn add_to_cell(&self) {
        print!("{}", "+".repeat(*self as usize));
    }
}

impl Pointer {
    fn shift_pointer(&mut self, amount: isize) {
        if (((*self).index as isize) + amount) < 0 {
            panic!("Pointer index cannot be negative.")
        }

        (*self).index = (((*self).index as isize) + amount) as u8;

        if amount > 0 {
            print!("{}", ">".repeat(amount as usize));
        } else {
            print!("{}", "<".repeat(amount.abs() as usize));
        }
    }

    fn to_pointer(&mut self, index: u8) {
        let shift: isize = (index as isize) - ((*self).index as isize);
        self.shift_pointer(shift);
    }

    pub fn write_char(&mut self, char: char) {
        self.push_char(char, self.stack_index);
        self.stack_index += 1;
    }

    fn push_char(&mut self, char: char, index: u8) {
        use crate::data_conversions::CharExtensions;

        self.stack.insert(index as usize, char);
        self.to_pointer(index);
        (char.to_ascii() as i8).add_to_cell();
    }
    pub fn print_char(&mut self, index: u8) {
        self.to_pointer(index);
        print!(".");
    }
}

