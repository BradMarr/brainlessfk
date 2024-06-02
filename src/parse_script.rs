use std::fs;
use crate::Pointer;

impl Pointer {
    /// Splits file contents at @path into a vector of strings.
    pub fn split_to_line_vector(&mut self, path: &str) -> Vec<String> {
        let mut contents = fs::read_to_string(path).expect("Something went wrong reading the file");

        contents = contents.replace("\r", "");
        return contents.split('\n').map(|s| s.to_string()).collect();
    }
}

pub trait StringExtensions {
    fn split_args(&mut self) -> Vec<&str>;
}

impl StringExtensions for String {
    /// Splits string into vector of arguments.
    fn split_args(&mut self) -> Vec<&str> {
        let mut result: Vec<&str> = Vec::new();
        let split_strings: Vec<&str> = self.split('\"').collect();

        if split_strings.len() == 1 {   //if no string
            return split_strings[0].split_whitespace().collect();
        }

        for i in 0..split_strings.len() - 1 {
            if i % 2 == 0 {
                result.append(&mut split_strings[i].trim().split_whitespace().collect());
            } else {
                result.push(split_strings[i]);
            }
        }

        return result;
    }
}

pub trait VecStrExtensions {
    fn exec_command(&mut self, pointer: &mut Pointer);
}

impl VecStrExtensions for Vec<&str> {
    /// Executes each argument in vector.
    fn exec_command(&mut self, pointer: &mut Pointer) {
        if self.len() == 0 {
            return;
        }
        match self[0] {
            "def" => pointer.def_var(self[1], self[2]),
            "print" => {
                match self[1] {
                    "var" => pointer.print_var(self[2]),
                    "str" => pointer.print_str(self[2]),
                    arg => panic!("Invalid print argument: {}", arg),
                }
            }
            command => panic!("Invalid command: {}", command),
        }
    }
}