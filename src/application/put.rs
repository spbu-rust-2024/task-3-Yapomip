use std::fs;
use std::io;
use std::path;

pub enum PutFrom {
    Console,
    File(String),
}

pub struct Put {
    output: PutFrom,
}

impl Put {
    pub fn new() -> Put {
        Put {
            output: PutFrom::Console,
        }
    }

    pub fn data_in(&self, status_in: PutFrom) -> Result<Vec<i64>, String> {
        let mut input = String::new();

        if let PutFrom::File(string_file_path) = status_in {
            let path = path::Path::new(&string_file_path);
            let display = path.display();

            input = match fs::read_to_string(path) {
                Ok(s) => s,
                Err(_) => return Err(format!("error of read \"{}\" file ", display)),
            };
        } else {
            println!("Please input the data:");

            io::stdin()
                .read_line(&mut input)
                .expect("failed to read file");
        }

        let data = input
            .split_whitespace()
            .map(|x| {
                match x.parse::<i64>() {
                    Ok(x) => Ok(x),
                    Err(_) => Err(format!("error of parse \"{}\" to i64", x))
                }
            })
            .collect::<Result<Vec<i64>, String>>();

        data
    }

    pub fn set_out(&mut self, status_out: PutFrom) {
        self.output = status_out;
    }
    
    pub fn data_out(&self, out: String) {
        if let PutFrom::File(string_file_path) = &self.output {
            fs::write(string_file_path, out)
                .expect(&format!("Failed write to {}", string_file_path));
        } else {
            println!("{}", out);
        }
    }
}
