use crate::color_out::*;

use std::fs;
use std::io;
use std::path;
use clap::*;

enum PutFrom {
    Console,
    File(String),
}

pub struct Put {
    input: Option<PutFrom>,
    output: PutFrom,
    
    data_in: Vec<i64>,
}

impl Put {
    pub fn new() -> Put {
        Put {
            input: None,
            output: PutFrom::Console,
            data_in: Vec::new()
        }
    }
    
    pub fn add_args(cmd: Command) -> Command {
        let new_cmd = cmd.clone()
            .args(&[
                arg!(-i --input [FILE] "input file name, else console input")
                    // .value_hint(ValueHint::FilePath)
                    .value_parser(clap::builder::NonEmptyStringValueParser::new()),
                arg!(-o --output [FILE] "output file name, else console output")
                    // .value_hint(ValueHint::FilePath)
                    .value_parser(clap::builder::NonEmptyStringValueParser::new()),
            ]);

        new_cmd
    }

    fn set_in(&mut self, in_option: Option<&String>) {
        if let Some(in_file_name) = in_option {
            if path::Path::new(in_file_name).exists() {
                self.input = Some(PutFrom::File(in_file_name.clone()));
                println!("{}", message(format!("set input file '{}'", in_file_name)));
            } else {
                println!("{}", error(format!("set input file fail, '{in_file_name}' not found")));
            }
        } else {
            self.input = Some(PutFrom::Console);
        }
    }
    
    fn set_out(&mut self, out_option: Option<&String>) {
        if let Some(out_file_name) = out_option {
            self.output = PutFrom::File(out_file_name.clone());
            println!("{}", message(format!("set output file '{}'", out_file_name)));
        } else {
            self.output = PutFrom::Console;
        }
    }
    
    pub fn update(&mut self, input_command: &ArgMatches) {
        if input_command.value_source("input") != None {
            self.set_in(input_command.get_one::<String>("input"));
        }
        if input_command.value_source("output") != None {
            self.set_out(input_command.get_one::<String>("output"));
        }
    }
    
    pub fn data_in<'a>(&'a mut self) -> &'a [i64] {
        let mut input = String::new();

        if let Some(put_from) = &self.input {
            match put_from {
                PutFrom::File(string_file_path) => {
                    let path = path::Path::new(&string_file_path);
                    let display = path.display();

                    if let Ok(s) = fs::read_to_string(path) {
                        input = s;
                    } else {
                        println!("{}", error(format!("cannot read \"{}\" file ", display)));
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
            
            let data_in_result = input
                .split_whitespace()
                .map(|x| {
                    match x.parse::<i64>() {
                        Ok(x) => Ok(x),
                        Err(_) => Err(format!("cannot parse \"{}\" to i64", x))
                    }
                })
                .collect::<Result<Vec<i64>, String>>();

            match data_in_result {
                Ok(data) => self.data_in = data,
                Err(err) => {
                    self.data_in = Vec::new();
                    println!("{}", error(err));
                },
            };
        }
        
        self.data_in.as_slice()
    }
    
    pub fn data_out(&self, out: String) {
        if out.len() > 0 {
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
}
