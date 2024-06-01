trait I8Extensions {
    fn add_to_cell(&self);
}

impl I8Extensions for i8 {
    fn add_to_cell(&self) {
        print!("{}", "+".repeat(*self as usize));
    }
}

pub trait U8Extensions {
    fn shift_pointer(&mut self, amount: isize);
    fn to_pointer(&mut self, index: u8);
    fn push_char(&mut self, char: char, index: u8);
    fn print_char(&mut self, index: u8);
}

impl U8Extensions for u8 {
    fn shift_pointer(&mut self, amount: isize) {
        if ((*self as isize) + amount) < 0 {
            panic!("Pointer index cannot be negative.")
        }

        *self = ((*self as isize) + amount) as u8;

        if amount > 0 {
            print!("{}", ">".repeat(amount as usize));
        } else {
            print!("{}", "<".repeat(amount.abs() as usize));
        }
    }

    fn to_pointer(&mut self, index: u8) {
        let shift: isize = (index as isize) - (*self) as isize;
        self.shift_pointer(shift);
    }

    fn push_char(&mut self, char: char, index: u8) {
        use crate::data_conversions::CharExtensions;

        self.to_pointer(index);
        (char.to_ascii() as i8).add_to_cell();
    }
    fn print_char(&mut self, index: u8) {
        self.to_pointer(index);
        print!(".");
    }
}

