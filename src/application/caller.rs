mod generalized_mean;
mod arithmetic_geometric_mean;
mod modified_arithmetic_geometric_mean;
mod truncated_mean;
mod winsorizing_mean;

use crate::color_out::*;
use calculate_lib::*;

use clap::*;

macro_rules! function_string_name {
    ( $function_expression:expr ) => {
        stringify!($function_expression).replace("_", " ")
    };
    ( $( $function_expression:expr )* $(,)? ) => {
        $(
            stringify!($function_expression).replace("_", " "),
        )*
    };
}

macro_rules! list {
    ( $( $function:expr, )* $(,)? ) => {
        {
            let mut tmp: u64 = 0;
            
            $(
                tmp += 1;
                println!("-{}               {}", tmp, function_string_name!($function));
            )*
        }
    }
}

pub fn list() {
    list!(
        arithmetic_mean,
        geometric_mean,
        generalized_mean,
        arithmetic_geometric_mean,
        modified_arithmetic_geometric_mean,
        quasi_arithmetic_mean,
        truncated_mean,
        winsorizing_mean,
        median_mean,
        moda_mean,
        average_linear_deviation,
        average_quadratic_deviation,
        linear_coefficient_variation,
        quadratic_coefficient_variation,
        variance,
    ); 
}

macro_rules! func_name {
    ( $f_name:expr ) => {
        stringify!($f_name).replace("_", " ")
    };
}
macro_rules! create_arg {
    ( $f:expr, $short:expr, $help:expr, $action:expr $(,)? ) => {
        {
            Arg::new($f)
                .short($short)
                .long($f)
                .help($help)
                .action($action)
        }
    };
    ( $short:literal, $f:expr $(,)? ) => {
        create_arg!(stringify!($f), $short, func_name!($f), ArgAction::SetTrue)
    };
}

pub const NUM_CALL_FUNCTIONS: usize = 15;

pub struct Caller {
    call_parametrs: [bool; NUM_CALL_FUNCTIONS],
    
    generalized_mean_value: u32,
    arithmetic_geometric_mean_value: f64,
    modified_arithmetic_geometric_mean_value: f64,
    truncated_mean_value: f64,
    winsorizing_mean_value: f64,
    
}

impl Caller {
    pub fn new() -> Caller {
        Caller {
            call_parametrs: [false; NUM_CALL_FUNCTIONS],
            
            generalized_mean_value: generalized_mean::DEFOULT,
            arithmetic_geometric_mean_value: arithmetic_geometric_mean::DEFOULT,
            modified_arithmetic_geometric_mean_value: modified_arithmetic_geometric_mean::DEFOULT,
            truncated_mean_value: truncated_mean::DEFOULT,
            winsorizing_mean_value: winsorizing_mean::DEFOULT,
        }
    }
    
    pub fn add_args(cmd: Command) -> Command {
        let args = [
            create_arg!('1', arithmetic_mean),
            create_arg!('2', geometric_mean),
            
            create_arg!("generalized_mean", '3', "generalized mean (value from 1 to 100)", ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("d")
                    .value_parser(value_parser!(u32)),
            create_arg!("arithmetic_geometric_mean", '4', "arithmetic geometric mean (epsilon for accuracy)", ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("epsilon")
                    .value_parser(value_parser!(f64)),
            create_arg!("modified_arithmetic_geometric_mean", '5', "modified arithmetic geometric mean (epsilon for accuracy)", ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("epsilon")
                    .value_parser(value_parser!(f64)),
            
            create_arg!('6', quasi_arithmetic_mean),

            create_arg!("truncated_mean", '7', "truncated mean (p - procent to cancel)", ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("p")
                    .value_parser(value_parser!(f64)),
            create_arg!("winsorizing_mean", '8', "winsorizing mean (p - procent to cancel)", ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("p")
                    .value_parser(value_parser!(f64)),
            
            create_arg!('a', median_mean),
            create_arg!('9', moda_mean),
            create_arg!('b', average_linear_deviation),
            create_arg!('c', average_quadratic_deviation),
            create_arg!('d', linear_coefficient_variation),
            create_arg!('e', quadratic_coefficient_variation),
            create_arg!('f', variance),
        ];
        let new_cmd = cmd.clone().args(&args);
        
        new_cmd
    }
    
    pub fn update(&mut self, input_command: &ArgMatches) {
         if input_command.get_flag("arithmetic_mean") {
             self.call_parametrs[0] = true;
         }
         
         if input_command.get_flag("geometric_mean") {
             self.call_parametrs[1] = true;
         }
         
         if input_command.value_source("generalized_mean") != None {
             self.call_parametrs[2] = true;
             if let Some(get_value) = input_command.get_one::<u32>("generalized_mean") {
                 self.generalized_mean_value = *get_value;
             }
         }
         if input_command.value_source("arithmetic_geometric_mean") != None {
             self.call_parametrs[3] = true;
             if let Some(get_value) = input_command.get_one::<f64>("arithmetic_geometric_mean") {
                 self.arithmetic_geometric_mean_value = *get_value;
             }
         }
         if input_command.value_source("modified_arithmetic_geometric_mean") != None {
             self.call_parametrs[4] = true;
             if let Some(get_value) = input_command.get_one::<f64>("modified_arithmetic_geometric_mean") {
                 self.modified_arithmetic_geometric_mean_value = *get_value;
             }
         }
         if input_command.get_flag("quasi_arithmetic_mean") {
             self.call_parametrs[5] = true;
         }
         if input_command.value_source("truncated_mean") != None {
             self.call_parametrs[6] = true;
             if let Some(get_value) = input_command.get_one::<f64>("truncated_mean") {
                 self.truncated_mean_value = *get_value;
             }
         }
         if input_command.value_source("winsorizing_mean") != None {
             self.call_parametrs[7] = true;
             if let Some(get_value) = input_command.get_one::<f64>("winsorizing_mean") {
                 self.winsorizing_mean_value = *get_value;
             }
         }
         
         if input_command.get_flag("median_mean") {
             self.call_parametrs[8] = true;
         }
         if input_command.get_flag("moda_mean") {
             self.call_parametrs[9] = true;
         }
         if input_command.get_flag("average_linear_deviation") {
             self.call_parametrs[10] = true;
         }
         if input_command.get_flag("average_quadratic_deviation") {
             self.call_parametrs[11] = true;
         }
         if input_command.get_flag("linear_coefficient_variation") {
             self.call_parametrs[12] = true;
         }
         if input_command.get_flag("quadratic_coefficient_variation") {
             self.call_parametrs[13] = true;
         }
         if input_command.get_flag("variance") {
             self.call_parametrs[14] = true;
         }
    }
    
    pub fn play(&mut self, data: &[i64]) -> String {
        let mut answer = String::new();
        
        if self.call_parametrs[0] {
            answer += &format!("arithmetic mean: {}", arithmetic_mean(data));
        }
        if self.call_parametrs[1] {
            answer += &format!("geometric mean: {}", geometric_mean(data));
        }
        if self.call_parametrs[2] {
            answer += &generalized_mean::call(data, self.generalized_mean_value);
        }
        if self.call_parametrs[3] {
            answer += &arithmetic_geometric_mean::call(data, self.arithmetic_geometric_mean_value);
        }
        if self.call_parametrs[4] {
            answer += &modified_arithmetic_geometric_mean::call(data, self.modified_arithmetic_geometric_mean_value);
        }
        if self.call_parametrs[5] {
            let fi = |x: f64| -> f64 { x.exp() + 2.0 * x * x + x + 5.0 };
            
            answer += &format!("quasi arithmetic (kolmogorov) mean: {}", quasi_arithmetic_mean(data, fi));
        }
        if self.call_parametrs[6] {
            answer += &truncated_mean::call(data, self.truncated_mean_value);
        }
        if self.call_parametrs[7] {
            answer += &winsorizing_mean::call(data, self.winsorizing_mean_value);
        }
        
        if self.call_parametrs[8] {
            answer += &format!("median mean: {}", median_mean(data));
        }
        if self.call_parametrs[9] {
            answer += &format!("moda mean: {}", moda_mean(data));
        }
        if self.call_parametrs[10] {
            answer += &format!("average linear deviation: {}", average_linear_deviation(data));
        }
        if self.call_parametrs[11] {
            answer += &format!("average quadratic deviation: {}", average_quadratic_deviation(data));
        }
        if self.call_parametrs[12] {
            answer += &format!("linear coefficient variation: {}", linear_coefficient_variation(data));
        }
        if self.call_parametrs[13] {
            answer += &format!("quadratic coefficient variation: {}", quadratic_coefficient_variation(data));
        }
        if self.call_parametrs[14] {
            answer += &format!("variance: {}", variance(data));
        }
        *self = Self::new();
        
        answer
    }
}