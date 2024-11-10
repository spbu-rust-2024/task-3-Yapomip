mod put;
use put::*;

mod caller;

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

    pub fn play(&mut self, input_command: &[&str]) {
        if input_command.cmp(["-h"].as_slice()) == Ordering::Equal
            || input_command.cmp(["--help"].as_slice()) == Ordering::Equal
        {
            println!("-h --help                     help application");
            println!("-i --input [FILENAME]         put filename for file (name not start from '-'), or non for console (default console)");
            println!("-o --output [FILENAME]        put filename for file (name not start from '-'), or non for console (default console)");
            println!("-l --list                     show all functions");
            caller::list();
        } else if input_command.cmp(["-l"].as_slice()) == Ordering::Equal
            || input_command.cmp(["--list"].as_slice()) == Ordering::Equal
        {
            caller::list();
        } else {
            let mut functions_parametrs: Vec<Vec<Option<&str>>> = vec![Vec::new(); caller::NUM_CALL_FUNCTIONS];

            let mut warnings = String::new();
            let mut i = 0;
            let mut data_option = None;
            
            while i < input_command.len() {
                if multi_campare!(input_command[i], "-i", "--input") {
                    /* if input parametr */
                    /* input command not void string bc split_witespaces */
                    if i + 1 < input_command.len() && !input_command[i + 1].starts_with("-") { 
                        /* if next exist & next not parametr*/
                        data_option = Some(self.put.data_in(PutFrom::File(input_command[i + 1].to_string())));
                        i += 1;
                    } else {
                        /* if next not exist */
                        data_option = Some(self.put.data_in(PutFrom::Console));
                    }
                } else if multi_campare!(input_command[i], "-o", "--output") {
                    /* if output parametr */
                    if i + 1 < input_command.len() && !input_command[i + 1].starts_with("-") {
                        /* if next exist & next not parametr*/
                        self.put.set_out(PutFrom::File(input_command[i + 1].to_string()));
                        i += 1;
                    } else {
                        /* if next not exist */
                        self.put.set_out(PutFrom::Console);
                    }
                } else {
                    /* if functions call parametrs */
                    if let Some(parametr) = input_command[i].strip_prefix("-") {
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
                                    /* if number less len */
                                    number_function -= 1;
                                    if i + 1 < input_command.len() && !input_command[i + 1].starts_with("-") {
                                        /* if next exist & not parametr */
                                        functions_parametrs[number_function].push(Some(input_command[i + 1]));
                                        i += 1;
                                    } else {
                                        functions_parametrs[number_function].push(None);
                                    }
                                } else {
                                    /* if number more len */
                                    warnings += &format!("error: number function out of range(1, {})\"{}\"\n", functions_parametrs.len() + 1, number_function);
                                }
                            } else {
                                /* if not number */
                                warnings += &format!("error: unknown parametr \"{}\"\n", parametr);
                            }
                        }
                    } else {
                        warnings += &format!("error: unknown expresion \"{}\"\n", input_command[i]);
                    }
                }
                
                i += 1;
            }
            
            if warnings.len() > 0 {
                println!("------------------------------");
                println!("Parse parametrs warnings:\n{}", warnings);
                println!("------------------------------");
            }
            if let Some(data_result) = data_option {
                match data_result {
                    Ok(new_data) => self.data = new_data,
                    Err(error_string) => {
                        println!("------------------------------");
                        println!("{}", error_string);
                        println!("------------------------------");
                    }
                }
            }
            
            
            let mut answer = String::new();
            
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
