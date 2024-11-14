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
    
    fn parse_input(&mut self, input_command: &[&str], active_number: &mut usize) -> bool {
        let mut answer: bool = false;
        
        /* if 'input' parametr */
        if multi_campare!(input_command[*active_number], "-i", "--input") {
            /* set true answer */
            answer = true;
            /* set active next */
            *active_number += 1;
            /* if input parametr */
            /* input command not void string bc split_witespaces */
            if *active_number < input_command.len() && !input_command[*active_number].starts_with("-") { 
                /* if next exist & next not parametr*/
                self.put.set_in(Some(input_command[*active_number].to_string()));
                /* set active next */
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
        
        /* if 'output' parametr */
        if multi_campare!(input_command[*active_number], "-o", "--output") {
            /* set true answer */
            answer = true;
            /* set active next */
            *active_number += 1;
            /* input command not void string bc split_witespaces */
            if *active_number < input_command.len() && !input_command[*active_number].starts_with("-") {
                /* if next exist & next not parametr*/
                self.put.set_out(Some(input_command[*active_number].to_string()));
                /* set active next */
                *active_number += 1;
            } else {
                /* if next not exist */
                self.put.set_out(None);
            }
        }
        
        answer
    }
    
    fn parse_called_functions<'a>(&mut self, input_command: &[&'a str], active_number: &mut usize, functions_parametrs: &mut [Vec<Option<&'a str>>]) -> bool {
        let mut answer: bool = false;

        /* if parametr */
        if let Some(parametr) = input_command[*active_number].strip_prefix("-") {
            /* if all */
            if parametr == "all" {
                /* set true answer */
                answer = true;
                
                /* set active next */
                *active_number += 1;
                /* all function turn */
                for i in 0..functions_parametrs.len() {
                    /* add call for all */
                    functions_parametrs[i].push(None);
                }
            } else {
                /* if number of function */
                if let Ok(mut number_function) = parametr.parse::<usize>() {
                    /* get number function */
                    if number_function > 0 && number_function < functions_parametrs.len() + 1 {
                        /* if parametr correct */
                        
                        /* set true answer */
                        answer = true;
                        /* set active next */
                        *active_number += 1;
                        /* if number less len */
                        number_function -= 1;
                        if *active_number < input_command.len() && !input_command[*active_number].starts_with("-") {
                            /* if next exist & not parametr */
                            functions_parametrs[number_function].push(Some(input_command[*active_number]));
                            /* set active next */
                            *active_number += 1;
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
            println!("-{}               {}", "all", "all functions start with default value");
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_parse_input() {
        let mut app = App::new();
        
        let mut active_number: usize = 0;
        let commands = [
            "-i",
            "in.txt",
            "--input",
            "in.txt",
            "--input",
            "-i",
            "-balbalbal",
            "balbalbal",
        ];
        
        assert_eq!(app.parse_input(commands.as_slice(), &mut active_number), true, "commands: '{}' '{}'", commands[0], commands[1]);
        assert_eq!(active_number, 2);

        assert_eq!(app.parse_input(commands.as_slice(), &mut active_number), true, "commands: '{}' '{}'", commands[2], commands[3]);
        assert_eq!(active_number, 4);

        assert_eq!(app.parse_input(commands.as_slice(), &mut active_number), true, "commands: '{}'", commands[4]);
        assert_eq!(active_number, 5);
        
        assert_eq!(app.parse_input(commands.as_slice(), &mut active_number), true, "commands: '{}'", commands[5]);
        assert_eq!(active_number, 6);

        assert_eq!(app.parse_input(commands.as_slice(), &mut active_number), false, "commands: '{}'", commands[6]);
        assert_eq!(active_number, 6);
    }

    #[test]
    fn test_app_parse_output() {
        let mut app = App::new();
        
        let mut active_number: usize = 0;
        let commands = [
            "-o",
            "out.txt",
            "--output",
            "out.txt",
            "--output",
            "-o",
            "-balbalbal",
            "balbalbal",
        ];
        
        assert_eq!(app.parse_output(commands.as_slice(), &mut active_number), true, "commands: '{}' '{}'", commands[0], commands[1]);
        assert_eq!(active_number, 2);

        assert_eq!(app.parse_output(commands.as_slice(), &mut active_number), true, "commands: '{}' '{}'", commands[2], commands[3]);
        assert_eq!(active_number, 4);

        assert_eq!(app.parse_output(commands.as_slice(), &mut active_number), true, "commands: '{}'", commands[4]);
        assert_eq!(active_number, 5);
        
        assert_eq!(app.parse_output(commands.as_slice(), &mut active_number), true, "commands: '{}'", commands[5]);
        assert_eq!(active_number, 6);

        assert_eq!(app.parse_output(commands.as_slice(), &mut active_number), false, "commands: '{}'", commands[6]);
        assert_eq!(active_number, 6);
    }
    #[test]
    fn test_app_parse_called_functions() {
        let mut app = App::new();
        
        let mut active_number: usize = 0;
        let commands = [
            "-3",
            "5",
            "-4",
            "1000",
            "-10",
            "-all",
            "-balbalbal",
            "balbalbal",
        ];
        let mut functions_parametrs: Vec<Vec<Option<&str>>> = vec![Vec::new(); 10];
        
        assert_eq!(app.parse_called_functions(commands.as_slice(), &mut active_number, &mut functions_parametrs), true, 
            "commands: '{}' '{}'", commands[0], commands[1]);
        assert_eq!(active_number, 2, "commands: '{}' '{}'", commands[0], commands[1]);

        assert_eq!(app.parse_called_functions(commands.as_slice(), &mut active_number, &mut functions_parametrs), true, 
            "commands: '{}' '{}'", commands[2], commands[3]);
        assert_eq!(active_number, 4, "commands: '{}' '{}'", commands[2], commands[3]);

        assert_eq!(app.parse_called_functions(commands.as_slice(), &mut active_number, &mut functions_parametrs), true, 
            "commands: '{}'", commands[4]);
        assert_eq!(active_number, 5, "commands: '{}'", commands[4]);
        
        assert_eq!(app.parse_called_functions(commands.as_slice(), &mut active_number, &mut functions_parametrs), true, 
            "commands: '{}'", commands[5]);
        assert_eq!(active_number, 6, "commands: '{}'", commands[5]);

        assert_eq!(app.parse_called_functions(commands.as_slice(), &mut active_number, &mut functions_parametrs), false, 
            "commands: '{}'", commands[6]);
        assert_eq!(active_number, 6, "commands: '{}'", commands[6]);
    }
}