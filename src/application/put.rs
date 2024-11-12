use crate::color_out::*;

use std::fs;
use std::io;
use std::path;

enum PutFrom {
    Console,
    File(String),
}

pub struct Put {
    input: Option<PutFrom>,
    output: PutFrom,
}

impl Put {
    pub fn new() -> Put {
        Put {
            input: None,
            output: PutFrom::Console,
        }
    }

    pub fn set_in(&mut self, in_option: Option<String>) {
        if let Some(in_file_name) = in_option {
            if path::Path::new(&in_file_name).exists() {
                self.input = Some(PutFrom::File(in_file_name));
            } else {
                error(format!("file {in_file_name} not found"));
            }
        } else {
            self.input = Some(PutFrom::Console);
        }
    }
    
    pub fn data_in(&mut self) -> Option<Result<Vec<i64>, String>> {
        let mut input = String::new();

        if let Some(put_from) = &self.input {
            match put_from {
                PutFrom::File(string_file_path) => {
                    let path = path::Path::new(&string_file_path);
                    let display = path.display();

                    if let Ok(s) = fs::read_to_string(path) {
                        input = s;
                    } else {
                        return Some(Err(format!("cannot read \"{}\" file ", display)));
                    }
                },
                PutFrom::Console => {
                    println!("Please input the data:");

                    io::stdin()
                        .read_line(&mut input)
                        .expect("failed to read file");
                }
            }
            self.input = None;
            
            let data = input
                .split_whitespace()
                .map(|x| {
                    match x.parse::<i64>() {
                        Ok(x) => Ok(x),
                        Err(_) => Err(format!("cannot parse \"{}\" to i64", x))
                    }
                })
                .collect::<Result<Vec<i64>, String>>();

            Some(data)
        } else {
            return None;
        }
    }

    pub fn set_out(&mut self, out_option: Option<String>) {
        if let Some(out_file_name) = out_option {
            self.output = PutFrom::File(out_file_name);
        } else {
            self.output = PutFrom::Console;
        }
    }
    
    pub fn data_out(&self, out: String) {
        if let PutFrom::File(string_file_path) = &self.output {
            match fs::write(string_file_path, out) {
                Ok(_) => println!("data succsesfully write"),
                Err(_) => println!("{}", error(format!("Failed write to {}", string_file_path)))
            }
        } else {
            println!("{}", out);
        }
    }
}
