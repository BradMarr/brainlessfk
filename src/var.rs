use crate::Pointer;

impl Pointer {
    /// Writes a variable to the var registry then inserts into stack.
    pub fn def_var(&mut self, name: &str, value: &str) {
        let ending_index: u16 = (value.len() as u16) + self.occupied_index;
        self.var_registry.insert(name.to_string(), self.occupied_index..ending_index);

        self.write_str(value);
    }

    /// Prints the variable with @name.
    pub fn print_var(&mut self, name: &str) {
        self.print_str_from_stack(self.var_registry.get(name).unwrap().clone());
    }
}