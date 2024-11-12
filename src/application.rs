mod put;
use put::*;

mod caller;

use crate::color_out::*;

use std::cmp::Ordering;

macro_rules! multi_campare {
    ( $f:expr, $first_cmp:expr, $( $cmp:expr )* $(,)? ) => {
        $f == $first_cmp $( || $f == $cmp )*
    };
}

pub struct App {
    put: Put,
    data: Vec<i64>,
    
}

impl App {
    pub fn new() -> App {
        App {
            put: Put::new(),
            data: Vec::new(),
        }
    }
    
    fn parse_input(&mut self, input_command: &[&str], active_number: &mut usize) -> bool{
        let mut answer: bool = false;
        
        if multi_campare!(input_command[*active_number], "-i", "--input") {
            /* if 'input' parametr */
            
            answer = true;
            /* set active next */
            *active_number += 1;
            /* if input parametr */
            /* input command not void string bc split_witespaces */
            if *active_number < input_command.len() && !input_command[*active_number].starts_with("-") { 
                /* if next exist & next not parametr*/
                self.put.set_in(Some(input_command[*active_number].to_string()));
                *active_number += 1;
            } else {
                /* if next not exist */
                self.put.set_in(None);
            }
        }
        
        answer
    }
    fn parse_output(&mut self, input_command: &[&str], active_number: &mut usize) -> bool {
        let mut answer: bool = false;
        
        if multi_campare!(input_command[*active_number], "-o", "--output") {
            /* if 'output' parametr */
            
            answer = true;
            /* set active next */
            *active_number += 1;
            /* input command not void string bc split_witespaces */
            if *active_number < input_command.len() && !input_command[*active_number].starts_with("-") {
                /* set active next */
                *active_number += 1;
                /* if next exist & next not parametr*/
                self.put.set_out(Some(input_command[*active_number].to_string()));
            } else {
                /* if next not exist */
                self.put.set_out(None);
            }
        }
        
        answer
    }
    
    fn parse_called_functions<'a>(&mut self, input_command: &[&'a str], active_number: &mut usize, functions_parametrs: &mut [Vec<Option<&'a str>>]) -> bool {
        let mut answer: bool = false;

        if let Some(parametr) = input_command[*active_number].strip_prefix("-") {
            /* number of function */
            if parametr == "all" {
                /* all function */
                for i in 0..functions_parametrs.len() {
                    /* add call for all */
                    functions_parametrs[i].push(None);
                }
            } else {
                /* if number of function */
                if let Ok(mut number_function) = parametr.parse::<usize>() {
                    /* get number function */
                    if number_function > 0 && number_function < functions_parametrs.len() + 1 {
                        
                        answer = true;
                        /* set active next */
                        *active_number += 1;
                        /* if number less len */
                        number_function -= 1;
                        if *active_number < input_command.len() && !input_command[*active_number].starts_with("-") {
                            /* if next exist & not parametr */
                            functions_parametrs[number_function].push(Some(input_command[*active_number]));
                        } else {
                            functions_parametrs[number_function].push(None);
                        }
                    } else {
                        /* if number more len */
                        println!("{}", warning(format!("error: number function out of range(1, {})\"{}\"\n", functions_parametrs.len() + 1, number_function)));
                    }
                } else {
                    /* if not number */
                    println!("{}", warning(format!("error: unknown parametr \"{}\"\n", parametr)));
                }
            }
        }
        
        answer
    }
    
    fn parse<'a>(&mut self, input_command: &[&'a str]) -> Option<Vec<Vec<Option<&'a str>>>> {
        if input_command.cmp(["-h"].as_slice()) == Ordering::Equal
            || input_command.cmp(["--help"].as_slice()) == Ordering::Equal
        {
            println!("-h --help                     help application");
            println!("-i --input [FILENAME]         put filename for file (name not start from '-'), or non for console (default console)");
            println!("-o --output [FILENAME]        put filename for file (name not start from '-'), or non for console (default console)");
            println!("-l --list                     show all functions");
            caller::list();
            None
        } else if input_command.cmp(["-l"].as_slice()) == Ordering::Equal
            || input_command.cmp(["--list"].as_slice()) == Ordering::Equal
        {
            println!("all functions:");
            caller::list();
            None
        } else {
            let mut functions_parametrs: Vec<Vec<Option<&str>>> = vec![Vec::new(); caller::NUM_CALL_FUNCTIONS];
            let mut i = 0;
            
            while i < input_command.len() {
                if self.parse_input(input_command, &mut i) ||
                    self.parse_output(input_command, &mut i) ||
                    self.parse_called_functions(input_command, &mut i, functions_parametrs.as_mut_slice())
                {
                } else {
                    warning(format!("error: unknown expresion \"{}\"\n", input_command[i]));
                }
            }
            
            Some(functions_parametrs)
        }
    }

    pub fn play(&mut self, input_command: &[&str]) {
        if let Some(functions_parametrs) = self.parse(input_command) {
            let mut answer = String::new();
            
            if let Some(new_data_result) = self.put.data_in() {
                match new_data_result {
                    Ok(new_data) => {
                        self.data = new_data;
                    },
                    Err(e) => println!("{}", error(e)),
                };
            }
            
            if self.data.len() == 0 {
                answer = "Data is not exist".to_string();
            } else {
                for i in 0..caller::NUM_CALL_FUNCTIONS {
                    answer += &caller::CALL_FUNCTIONS_MASS[i](self.data.as_slice(), &functions_parametrs[i]);
                }
            }
            
            self.put.data_out(answer);
        }
    }
}
