use crate::color_out::*;

use calculate_lib::*;

use paste::paste;

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

macro_rules! if_some_macro {
    ( $data:ident, $parametrs:ident, $param_string:ident : $if_some:expr => $else_some:expr ) => {
        if $data.len() > 0 {
            for param_option in $parametrs {
                if let Some($param_string) = param_option {
                    $if_some;
                } else {
                    $else_some;
                }
            }
        }
    };
    ( $data:ident, $parametrs:ident : $if_some:expr , $else_some:expr ) => {
        if $data.len() > 0 {
            for param_option in $parametrs {
                if let Some(param_string) = param_option {
                    $if_some;
                } else {
                    $else_some;
                }
            }
        }
    };
}

macro_rules! create_call_function {
    ( $function_name:ident, $func_string_name:expr, $func:expr ) => {
        fn $function_name(data: &[i64], parametrs: &Vec<Option<&str>>) -> String {
            let mut answer = String::new();
            
            if_some_macro!(data, parametrs, param_string :
                answer += &error(format!("'{}' : have no parametr '{}'!\n", $func_string_name, param_string))
                =>
                answer += &format!("{} : {}\n", $func_string_name, $func(data))
            );
            // if data.len() > 0 {
            //     for param_option in parametrs {
            //         if let Some(param_string) = param_option {
            //             answer += &error(format!("'{}' : have no parametr '{}'!\n", $func_string_name, param_string));
            //         } else {
            //             answer += &format!("{} : {}\n", $func_string_name, $func(data));
            //         }
            //     }
            // }
            
            answer
        }
    };
    ( $( $f:expr ),* $(,)? ) => {
        paste! {
            $(
                create_call_function!([<call_ $f>], function_string_name!($f), $f);
            )*
        }
    };
}

create_call_function!(
    arithmetic_mean,
    geometric_mean,
    median_mean,
    moda_mean,
    average_linear_deviation,
    average_quadratic_deviation,
    linear_coefficient_variation,
    quadratic_coefficient_variation,
    variance
);

const GENERALIZED_MEAN_DEFOULT_VALUE: u32 = 3;
fn call_generalized_mean(data: &[i64], parametrs: &Vec<Option<&str>>) -> String {
    let mut answer = String::new();
    let function_name: String = function_string_name!(generalized_mean);
    
    if_some_macro!(data, parametrs, param_string :
        if let Ok(value) = param_string.parse::<u32>() {
            if value > 0 && value < 300 {
                answer += &format!(
                    "{} ({}) : {}\n",
                    function_name,
                    value,
                    generalized_mean(data, value)
                );
            } else {
                answer += &error(format!(
                    "'{}' ({}) : value must be more 0 and less 300\n",
                    function_name,
                    param_string,
                ));
            }
        } else {
            answer += &error(format!(
                "'{}' ({}) : can't parse to u32\n",
                function_name,
                param_string,
            ));
        }
        =>
        answer += &format!("{} (def: {}) : {}\n",
            function_name,
            GENERALIZED_MEAN_DEFOULT_VALUE,
            generalized_mean(data, GENERALIZED_MEAN_DEFOULT_VALUE)
        )
    );
    
    answer
}

const ARITHMETIC_GEOMETRIC_MEAN_DEFOULT_VALUE: u32 = 100;
fn call_arithmetic_geometric_mean(data: &[i64], parametrs: &Vec<Option<&str>>) -> String {
    let mut answer = String::new();
    let function_name: String = function_string_name!(arithmetic_geometric_mean);
    
    if_some_macro!(data, parametrs, param_string :
        if let Ok(value) = param_string.parse::<u32>() {
            if value > 50 && value < 10000 {
                answer += &format!("{} ({}) : \n", function_name, value);
                arithmetic_geometric_mean(data, value)
                    .iter()
                    .enumerate()
                    .for_each(|(i, x)| answer += &format!("    {} {} : {}\n", data[i], data[i + 1], *x));
            } else {
                answer += &error(format!(
                    "'{}' ({}) : value must be more 50 and less 10000\n",
                    function_name,
                    param_string,
                ));
            }
        } else {
            answer += &error(format!(
                "'{}' ({}) : can't parse to u32\n",
                function_name,
                param_string,
            ));
        }
        =>
        {
            answer += &format!("{} (def: {}) : \n", function_name, ARITHMETIC_GEOMETRIC_MEAN_DEFOULT_VALUE);
            arithmetic_geometric_mean(data, ARITHMETIC_GEOMETRIC_MEAN_DEFOULT_VALUE)
                .iter()
                .enumerate()
                .for_each(|(i, x)| answer += &format!("    {} {} : {}\n", data[i], data[i + 1], *x));
        }
    );
    
    answer
}
const MODIFIED_ARITHMETIC_GEOMETRIC_MEAN_DEFOULT_VALUE: u32 = 100;
fn call_modified_arithmetic_geometric_mean(data: &[i64], parametrs: &Vec<Option<&str>>) -> String {
    let mut answer = String::new();
    let function_name: String = function_string_name!(modified_arithmetic_geometric_mean);
    
    if_some_macro!(data, parametrs, param_string :
        if let Ok(value) = param_string.parse::<u32>() {
            if value > 50 && value < 10000 {
                answer += &format!("{} ({}) : \n", function_name, value);
                modified_arithmetic_geometric_mean(data, value)
                    .iter()
                    .enumerate()
                    .for_each(|(i, x)| answer += &format!("    {} {} : {}\n", data[i], data[i + 1], *x));
            } else {
                answer += &error(format!(
                    "'{}' ({}) : value must be more 50 and less 10000\n",
                    function_name,
                    param_string,
                ));
            }
        } else {
            answer += &error(format!(
                "'{}' ({}) : can't parse to u32\n",
                function_name,
                param_string,
            ));
        }
        =>
        {
            answer += &format!("{} (def: {}) : \n", function_name, MODIFIED_ARITHMETIC_GEOMETRIC_MEAN_DEFOULT_VALUE);
            modified_arithmetic_geometric_mean(data, MODIFIED_ARITHMETIC_GEOMETRIC_MEAN_DEFOULT_VALUE)
                .iter()
                .enumerate()
                .for_each(|(i, x)| answer += &format!("    {} {} : {}\n", data[i], data[i + 1], *x));
        }
    );

    answer
}

const QUASI_ARITHMETIC_MEAN_DEFOULT_VALUE: usize = 1;
fn call_quasi_arithmetic_mean(data: &[i64], parametrs: &Vec<Option<&str>>) -> String {
    let mut answer = String::new();
    let function_name: String = function_string_name!(quasi_arithmetic_mean);
    let functions = [
        |x: f64| -> f64 {
            x.exp() + 2.0 * x * x + x + 5.0 
        },
    ];
    
    if_some_macro!(data, parametrs, param_string :
        if let Ok(value) = param_string.parse::<usize>() {
            if value > 0 && value < functions.len() + 1 {
                answer += &format!("{} ({}) : {}\n", 
                    function_name, 
                    value, 
                    quasi_arithmetic_mean(data, functions[value - 1])
                );
            } else {
                answer += &error(format!(
                    "'{}' ({}) : value must be more 1 and less {}\n",
                    function_name,
                    param_string,
                    functions.len() + 1
                ));
            }
        } else {
            answer += &error(format!(
                "'{}' ({}) : can't parse to usize\n",
                function_name,
                param_string,
            ));
        }
        =>
        answer += &format!("{} (def: {}) : {}\n", 
            function_name, 
            QUASI_ARITHMETIC_MEAN_DEFOULT_VALUE, 
            quasi_arithmetic_mean(data, functions[QUASI_ARITHMETIC_MEAN_DEFOULT_VALUE - 1])
        )
    );
    
    answer
}

const TRUNCATED_MEAN_DEFOULT_VALUE: f64 = 0.20;
fn call_truncated_mean(data: &[i64], parametrs: &Vec<Option<&str>>) -> String {
    let mut answer = String::new();
    let function_name: String = function_string_name!(truncated_mean);
    
    if_some_macro!(data, parametrs, param_string :
        if let Ok(value) = param_string.parse::<f64>() {
            if value > 0.0 && value < 0.49 {
                answer += &format!("{} ({}) : {}\n", 
                    function_name, 
                    value, 
                    truncated_mean(data, value)
                );
            } else {
                answer += &error(format!(
                    "'{}' ({}) : value must be more 0.0 and less 0.49\n",
                    function_name,
                    param_string,
                ));
            }
        } else {
            answer += &error(format!(
                "'{}' ({}) : can't parse to f64\n",
                function_name,
                param_string,
            ));
        }
        =>
        answer += &format!("{} (def: {}) : {}\n", 
            function_name, 
            TRUNCATED_MEAN_DEFOULT_VALUE, 
            truncated_mean(data, TRUNCATED_MEAN_DEFOULT_VALUE)
        )
    );
    
    answer
}

const WINSORIZING_MEAN_DEFOULT_VALUE: f64 = 0.20;
fn call_winsorizing_mean(data: &[i64], parametrs: &Vec<Option<&str>>) -> String {
    let mut answer = String::new();
    let function_name: String = function_string_name!(winsorizing_mean);
    
    if_some_macro!(data, parametrs, param_string :
        if let Ok(value) = param_string.parse::<f64>() {
            if value > 0.0 && value < 0.49 {
                answer += &format!("{} ({}) : {}\n", 
                    function_name, 
                    value, 
                    winsorizing_mean(data, value)
                );
            } else {
                answer += &error(format!(
                    "'{}' ({}) : value must be more 0.0 and less 0.49\n",
                    function_name,
                    param_string,
                ));
            }
        } else {
            answer += &error(format!(
                "'{}' ({}) : can't parse to f64\n",
                function_name,
                param_string,
            ));
        }
        =>
        answer += &format!("{} (def: {}) : {}\n", 
            function_name, 
            WINSORIZING_MEAN_DEFOULT_VALUE, 
            winsorizing_mean(data, WINSORIZING_MEAN_DEFOULT_VALUE)
        )
    );
    
    answer
}


pub const NUM_CALL_FUNCTIONS: usize = 15;
pub const CALL_FUNCTIONS_MASS: [fn (&[i64], &Vec<Option<&str>>) -> String; NUM_CALL_FUNCTIONS] = [
    call_arithmetic_mean,
    call_geometric_mean,
    call_generalized_mean,
    call_arithmetic_geometric_mean,
    call_modified_arithmetic_geometric_mean,
    call_quasi_arithmetic_mean,
    call_truncated_mean,
    call_winsorizing_mean,
    call_median_mean,
    call_moda_mean,
    call_average_linear_deviation,
    call_average_quadratic_deviation,
    call_linear_coefficient_variation,
    call_quadratic_coefficient_variation,
    call_variance
];


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
    println!("-{}               {}", "all", "all functions start");
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
