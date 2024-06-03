use crate::Pointer;
use crate::StackValue;

trait I8Extensions {
    fn add_to_cell(&self);
    fn subtract_from_cell(&self);
}

impl I8Extensions for i8 {
    /// Adds @self to cell.
    fn add_to_cell(&self) {
        print!("{}", "+".repeat(*self as usize));
    }
    /// Subtracts @self to cell.
    fn subtract_from_cell(&self) {
        print!("{}", "-".repeat(*self as usize));
    }
}

impl Pointer {
    /// Shifts pointer by @amount (negatives shift left).
    fn shift_pointer(&mut self, amount: isize) {
        if (((*self).index as isize) + amount) < 0 {
            panic!("Pointer index cannot be negative.")
        }

        (*self).index = (((*self).index as isize) + amount) as u16;

        if amount > 0 {
            print!("{}", ">".repeat(amount as usize));
        } else {
            print!("{}", "<".repeat(amount.abs() as usize));
        }
    }
    /// Moves pointer to @index.
    fn to_pointer(&mut self, index: u16) {
        let shift: isize = (index as isize) - ((*self).index as isize);
        self.shift_pointer(shift);
    }
    /// Pushes @char to stack at stack index then increments stack index.
    pub fn write_char(&mut self, char: char) {
        self.push_char(char, self.occupied_index);
        self.occupied_index += 1;
    }
    /// Inserts @char at @index into stack.
    pub fn push_char(&mut self, char: char, index: u16) {
        use crate::data_conversions::CharExtensions;

        let past_value = match self.stack[index as usize] {
            StackValue::Char(c) => c as i8,
            _ => panic!("Trying to overwrite ambiguous variable."),
        };

        let current_value = char.to_ascii() as i8;
        self.stack.insert(index as usize, crate::StackValue::Char(char));
        self.to_pointer(index);
        if current_value > past_value {
            (current_value - past_value).add_to_cell();
        } else {
            (past_value - current_value).subtract_from_cell();
        }
    }
    /// Prints the char at @index.
    pub fn print_char_from_stack(&mut self, index: u16) {
        self.to_pointer(index);
        print!(".");
    }
    /// Prints @value (streams through buffer).
    pub fn print_char(&mut self, value: char) {
        self.push_char(value, 0);
        self.print_char_from_stack(0);
    }
    /// Gets input of length @length and stores it as @name.
    pub fn get_input(&mut self, length: usize, name: &str) {
        self.to_pointer(self.occupied_index);
        print!("{}", ",>".repeat(length));
        let range_of_memory_use = self.occupied_index..(length as u16 + self.occupied_index);
        self.var_registry.insert(name.to_string(), range_of_memory_use.clone());
        for i in range_of_memory_use {
            self.stack.insert(i as usize, crate::StackValue::Var(true));
            self.occupied_index += 1;
            self.index += 1;
        }
    }
}