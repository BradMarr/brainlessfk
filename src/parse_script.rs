use std::fs;
use crate::Pointer;
use crate::var::Value;

impl Pointer {
    /// Splits file contents at @path into a vector of strings.
    pub fn split_to_line_vector(&mut self, path: &str) -> Vec<String> {
        let mut contents = fs::read_to_string(path).expect("Something went wrong reading the file");

        contents = contents.replace("\r", "");
        return contents.split('\n').map(|s| s.to_string()).collect();
    }
    /// Executes script at @path.
    pub fn exec_script(&mut self, path: &str) {
        for mut line in self.split_to_line_vector(path) {
            line.split_args().exec_command(self);
        }
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
                if split_strings[i].contains('#') {
                    return result;
                }
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
            "def" => {
                match self[1] {
                    "char" => pointer.def_var(self[2], Value::U8(self[3].parse::<u8>().expect("Invalid u8"))),
                    "str" => pointer.def_var(self[2], Value::Str(self[3])),
                    "input" => pointer.get_input(self[3].parse().expect(&format!("Invalid length of input: {}", self[3])), self[2]),
                    arg => panic!("Invalid def argument: {}", arg),
                }
            },
            "print" => {
                match self[1] {
                    "var" => pointer.print_var(self[2]),
                    "str" => pointer.print_str(self[2]),
                    "char" => {
                        let value: u8 = self[2].parse().unwrap();
                        pointer.print_char(value as char);
                    },
                    arg => panic!("Invalid print argument: {}", arg),
                }
            }
            command => panic!("Invalid command: {}", command),
        }
    }
}