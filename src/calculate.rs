
pub mod caller {
    use clap::*;
    use paste::paste;

    use crate::calculate::functions::*;

    macro_rules! func_name {
        ( $f_name:expr ) => {
            stringify!($f_name).replace("_", " ")
        };
    }

    macro_rules! call_function {
        ( $func_name:ident, $func_string_name:expr, $func:expr ) => {
            fn $func_name(data: &[i64]) -> String {
                format!("{} : {}\n", $func_string_name, $func(data))
            }
        };
        ( $( $f:expr ),* $(,)? ) => {
            paste! {
                $(
                    call_function!([<call_ $f>], func_name!($f), $f);
                )*
            }
        };
    }
    macro_rules! create_arg {
        ( $f:expr, $short:expr, $help:expr, $action:expr $(,)? ) => {
            Arg::new($f)
                .short($short)
                .long($f)
                .help($help)
                .action($action)
                .default_value_if("all_functions", "true", "true")
        };
        ( $short:expr, $f:expr $(,)? ) => {
            create_arg!(stringify!($f), $short, func_name!($f), ArgAction::SetTrue)
        };
    }
    macro_rules! if_matches_call_function {
        ( $data:expr, $ans:expr, $matches:expr, $( $f:expr ),* $(,)? ) => {
            $(
            if $matches.contains_id(stringify!($f)) && $matches.get_flag(stringify!($f)) {
            paste! {
                $ans += &[<call_ $f>]($data);
            }
            }
            )*
        };
    }

    call_function!(
        arithmetic_mean,
        geometric_mean,
        // generalized_mean,
        // arithmetic_geometric_mean,
        // modified_arithmetic_geometric_mean,
        // quasi_arithmetic_mean,
        // truncated_mean,
        // winsorizing_mean,
        median_mean,
        moda_mean,
        average_linear_deviation,
        average_quadratic_deviation,
        linear_coefficient_variation,
        quadratic_coefficient_variation,
        variance
    );

    fn call_generalized_mean(data: &[i64], d: u32) -> String {
        format!(
            "{} ({}) : {}\n",
            func_name!(generalized_mean),
            d,
            generalized_mean(data, d)
        )
    }
    fn call_arithmetic_geometric_mean(data: &[i64], n: u32) -> String {
        let mut ans = format!("{} ({}): \n", func_name!(arithmetic_geometric_mean), n);
        arithmetic_geometric_mean(data, n)
            .iter()
            .enumerate()
            .for_each(|(i, x)| ans += &format!("    {} {} : {}\n", data[i], data[i + 1], *x));

        ans
    }
    fn call_modified_arithmetic_geometric_mean(data: &[i64], n: u32) -> String {
        let mut ans = format!("{} : \n", func_name!(modified_arithmetic_geometric_mean));
        modified_arithmetic_geometric_mean(data, n)
            .iter()
            .enumerate()
            .for_each(|(i, x)| ans += &format!("    {} {} : {}\n", data[i], data[i + 1], *x));

        ans
    }
    fn call_quasi_arithmetic_mean(data: &[i64], fi: fn(x: f64) -> f64) -> String {
        format!(
            "{} : {}\n",
            func_name!(quasi_arithmetic_mean),
            quasi_arithmetic_mean(data, fi)
        )
    }
    fn call_truncated_mean(data: &[i64], procent: f64) -> String {
        format!(
            "{} : {}\n",
            func_name!(truncated_mean),
            truncated_mean(data, procent)
        )
    }
    fn call_winsorizing_mean(data: &[i64], procent: f64) -> String {
        format!(
            "{} : {}\n",
            func_name!(winsorizing_mean),
            winsorizing_mean(data, procent)
        )
    }

    pub fn add_args(cmd: Command) -> Command {
        let args_ = 
            [
                Arg::new("all_functions")
                    .long("all")
                    .help("all functions")
                    .action(ArgAction::SetTrue),
                
                create_arg!('1', arithmetic_mean),
                create_arg!('2', geometric_mean),
                
                Arg::new("generalized_mean")
                    .short('3')
                    .long("generalized_mean")
                    .help("generalized mean")
                    .action(ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("d")
                    .default_missing_value("3")
                    .value_parser(value_parser!(u32).range(1..=100))
                    .default_value_if("all_functions", "true", "3"),
                    
                Arg::new("arithmetic_geometric_mean")
                    .short('4')
                    .long("arithmetic_geometric_mean")
                    .help("arithmetic geometric mean")
                    .action(ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("n")
                    .default_missing_value("100")
                    .value_parser(value_parser!(u32).range(50..=1000))
                    .default_value_if("all_functions", "true", "100"),
                    
                Arg::new("modified_arithmetic_geometric_mean")
                    .short('5')
                    .long("modified_arithmetic_geometric_mean")
                    .help("modified arithmetic geometric mean")
                    .action(ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("n")
                    .default_missing_value("100")
                    .value_parser(value_parser!(u32).range(50..=1000))
                    .default_value_if("all_functions", "true", "100"),
                    
                Arg::new("quasi_arithmetic_mean")
                    .short('6')
                    .long("quasi_arithmetic_mean")
                    .help("quasi arithmetic mean")
                    .action(ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("f")
                    .default_missing_value("1")
                    .value_parser(value_parser!(usize))
                    .default_value_if("all_functions", "true", "1"),
                    
                Arg::new("truncated_mean")
                    .short('7')
                    .long("truncated_mean")
                    .help("truncated mean")
                    .action(ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("p")
                    .default_missing_value("0.20")
                    .value_parser(value_parser!(f64))
                    .default_value_if("all_functions", "true", "0.20"),
                    
                Arg::new("winsorizing_mean")
                    .short('8')
                    .long("winsorizing_mean")
                    .help("winsorizing mean")
                    .action(ArgAction::Append)
                    .num_args(0..=1)
                    .value_name("p")
                    .default_missing_value("0.20")
                    .value_parser(value_parser!(f64))
                    .default_value_if("all_functions", "true", "0.20"),

                create_arg!('9', median_mean),
                create_arg!('a', moda_mean),
                create_arg!('b', average_quadratic_deviation),
                create_arg!('c', average_linear_deviation),
                create_arg!('d', linear_coefficient_variation),
                create_arg!('e', quadratic_coefficient_variation),
                create_arg!('f', variance)
            ];
            
        let group_ = ArgGroup::new("functions").args([
                    "all_functions",
                    
                    "arithmetic_mean",
                    "geometric_mean",
                    "generalized_mean",
                    "arithmetic_geometric_mean",
                    "modified_arithmetic_geometric_mean",
                    "quasi_arithmetic_mean",
                    "truncated_mean",
                    "winsorizing_mean",
                    "median_mean",
                    "moda_mean",
                    "average_linear_deviation",
                    "average_quadratic_deviation",
                    "linear_coefficient_variation",
                    "quadratic_coefficient_variation",
                    "variance",
                ])
                .multiple(true);
                
        cmd.clone().args(args_).group(group_)
    }
    pub fn call(data: &[i64], matches: &ArgMatches) -> String {
        let mut ans = String::new();

        if_matches_call_function!(data, ans, matches, 
            arithmetic_mean,
            geometric_mean,
        );

        if let Some(all_d) = matches.get_many::<u32>("generalized_mean") {
            all_d
                .copied()
                .for_each(|d| ans += &call_generalized_mean(data, d));
        }
        if let Some(all_n) = matches.get_many::<u32>("arithmetic_geometric_mean") {
            all_n
                .copied()
                .for_each(|n| ans += &call_arithmetic_geometric_mean(data, n));
        }
        if let Some(all_n) = matches.get_many::<u32>("modified_arithmetic_geometric_mean") {
            all_n
                .copied()
                .for_each(|n| ans += &call_modified_arithmetic_geometric_mean(data, n));
        }
        if let Some(all_f) = matches.get_many::<usize>("quasi_arithmetic_mean") {
            let functions = [
                |x: f64| -> f64 {
                    x.exp() + 2.0 * x * x + x + 5.0 
                },
            ];
            all_f
                .copied()
                .for_each(|f| {
                    assert!(f > 0 && f <= functions.len());
                    ans += &call_quasi_arithmetic_mean(data, functions[f - 1]);
                });
        }
        if let Some(all_p) = matches.get_many::<f64>("truncated_mean") {
            all_p
                .copied()
                .for_each(|string_p| {
                    let p = string_p; // .parse::<f64>().expect("err");
                    assert!(p >= 0.0 && p <= 0.49);
                    ans += &call_truncated_mean(data, p);
                });
        }
        if let Some(all_p) = matches.get_many::<f64>("winsorizing_mean") {
            all_p
                .copied()
                .for_each(|string_p| {
                    let p = string_p; // .parse::<f64>().expect("err");
                    assert!(p >= 0.0 && p <= 0.49);
                    ans += &call_winsorizing_mean(data, p);
                });
        }
        
        if_matches_call_function!(data, ans, matches,
            median_mean,
            moda_mean,
            average_linear_deviation,
            average_quadratic_deviation,
            linear_coefficient_variation,
            quadratic_coefficient_variation,
            variance
        );
        
        ans
    }
}

pub mod functions {
    pub fn arithmetic_mean(data: &[i64]) -> f64 {
        let mut ans: f64 = 0.0;

        data.iter().for_each(|x| {
            ans += *x as f64;
        });
        ans /= data.len() as f64;

        ans
    }

    pub fn geometric_mean(data: &[i64]) -> f64 {
        let mut ans: f64 = 1.0;

        data.iter().for_each(|x| {
            ans *= *x as f64;
        });
        ans = ans.powf(1.0f64 / data.len() as f64);

        ans
    }

    // d = 3
    pub fn generalized_mean(data: &[i64], d: u32) -> f64 {
        let mut ans: f64 = 1.0;

        data.iter().for_each(|x| {
            ans *= x.pow(d) as f64;
        });
        ans /= data.len() as f64;
        ans = ans.powf(1.0f64 / data.len() as f64);

        ans
    }

    // n = 100
    pub fn arithmetic_geometric_mean(data: &[i64], n: u32) -> Vec<f64> {
        let mut ans: Vec<f64> = Vec::with_capacity(data.len() / 2);
        let mut a: f64 = 0.0;
        let mut b: f64 = 0.0;

        for i in 0..data.len() / 2 {
            a = data[i] as f64;
            b = data[i + 1] as f64;

            for _j in 0..n {
                let tmp = a;

                a = (tmp + b) / 2.0;
                b = (tmp + b).sqrt();
            }
            ans.push(a);
        }

        ans
    }

    // n = 100
    pub fn modified_arithmetic_geometric_mean(data: &[i64], n: u32) -> Vec<f64> {
        let mut ans: Vec<f64> = Vec::with_capacity(data.len() / 2);
        
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut z: f64 = 0.0;

        for i in 0..data.len() / 2 {
            x = data[i] as f64;
            y = data[i + 1] as f64;
            z = 0 as f64;

            for _j in 0..n {
                let tmp1 = x;
                let tmp2 = y;

                x = (tmp1 + tmp2) / 2.0;
                y = z + ((tmp1 - z) * (tmp2 - z)).sqrt();
                z = z - ((tmp1 - z) * (tmp2 - z)).sqrt();
            }
            ans.push(x);
        }

        ans
    }

    /// Kolmogorov mean
    // fi |x: f64| -> f64 { x.exp() + 2.0 * x * x + x + 5.0 };
    pub fn quasi_arithmetic_mean(data: &[i64], fi: fn(x: f64) -> f64) -> f64 {
        let mut ans: f64 = 1.0;

        data.iter().for_each(|x| {
            ans += fi(*x as f64);
        });
        ans /= data.len() as f64;
        ans = 1.0 / fi(ans);

        ans
    }

    pub mod sort {
        fn swap<T: Copy>(slice: &mut [T], n1: usize, n2: usize) {
            let tmp: T = slice[n1];

            slice[n1] = slice[n2];
            slice[n2] = tmp;
        }

        pub fn sort_shell(mass: &mut [i64]) {
            let mut s = mass.len() / 2;

            while s > 0 {
                for i in s..mass.len() {
                    let mut j = i - s;

                    loop {
                        if mass[j] > mass[j + s] {
                            swap::<i64>(mass, j, j + s);
                        }

                        if j >= s {
                            j = j - s;
                        } else {
                            break;
                        }
                    }
                }
                s = s / 2;
            }
        }
    }

    // procent 0.20
    pub fn truncated_mean(data: &[i64], procent: f64) -> f64 {
        let l: usize = (data.len() as f64 * procent) as usize;
        let r: usize = (data.len() as f64 * (1.0 - procent)) as usize;
        let mut data_copy = data[l..r].to_vec();

        sort::sort_shell(&mut data_copy);

        arithmetic_mean(&data_copy)
    }

    // procent 0.20
    pub fn winsorizing_mean(data: &[i64], procent: f64) -> f64 {
        let l: usize = (data.len() as f64 * procent) as usize;
        let r: usize = (data.len() as f64 * (1.0 - procent)) as usize;
        let mut data_copy = data.to_vec();

        sort::sort_shell(&mut data_copy);

        // let min = data_copy[l];
        // let max = data_copy[r];
        // data_copy.iter().enumerate().take_while(|(i, x)| *i < l).map(|x| min);
        // data_copy.iter().enumerate().skip_while(|(i, x)| *i <= r).map(|x| max);
        for i in 0..data_copy.len() {
            if i < l {
                data_copy[i] = data_copy[l];
            } else if i > r {
                data_copy[i] = data_copy[r];
            }
        }

        arithmetic_mean(&data_copy)
    }

    pub fn median_mean(data: &[i64]) -> f64 {
        let mut data_copy = data.to_vec();

        sort::sort_shell(&mut data_copy);
        if data_copy.len() % 2 == 1 {
            data_copy[data_copy.len() / 2 + 1] as f64
        } else {
            (data_copy[data_copy.len() / 2] + data_copy[data_copy.len() / 2]) as f64 / 2.0
        }
    }

    pub fn moda_mean(data: &[i64]) -> f64 {
        let mut data_copy = data.to_vec();
        let mut max_len_first_index: usize = 0;
        let mut max_len: u64 = 0;
        let mut f: usize = 0;
        // let mut len: u64 = 0;

        sort::sort_shell(&mut data_copy);
        for i in 0..data_copy.len() {
            if data_copy[i] != data_copy[f] {
                if max_len < (i - f).try_into().unwrap() {
                    max_len = (i - f).try_into().unwrap();
                    max_len_first_index = f;
                }
                f = i;
            }
        }

        data_copy[max_len_first_index] as f64
    }

    pub fn average_linear_deviation(data: &[i64]) -> f64 {
        let mut ans: f64 = 0.0;
        let x_ = arithmetic_mean(data);

        data.iter().for_each(|x| {
            ans += *x as f64 - x_;
        });
        ans /= data.len() as f64;

        ans
    }

    pub fn average_quadratic_deviation(data: &[i64]) -> f64 {
        let mut ans: f64 = 0.0;
        let x_ = arithmetic_mean(data);

        data.iter().for_each(|x| {
            ans += (*x as f64 - x_) * (*x as f64 - x_);
        });
        ans /= data.len() as f64;
        ans = ans.sqrt();

        ans
    }

    pub fn linear_coefficient_variation(data: &[i64]) -> f64 {
        let mut ans: f64 = 0.0;
        let x_ = arithmetic_mean(data);

        data.iter().for_each(|x| {
            ans += *x as f64 - x_;
        });
        ans /= data.len() as f64;
        ans /= x_;

        ans
    }

    pub fn quadratic_coefficient_variation(data: &[i64]) -> f64 {
        let mut ans: f64 = 0.0;
        let x_ = arithmetic_mean(data);

        data.iter().for_each(|x| {
            ans += (*x as f64 - x_) * (*x as f64 - x_);
        });
        ans /= data.len() as f64;
        ans = ans.sqrt();
        ans /= x_;

        ans
    }

    pub fn variance(data: &[i64]) -> f64 {
        let mut ans: f64 = 0.0;
        let x_ = arithmetic_mean(data);

        data.iter().for_each(|x| {
            ans += (*x as f64 - x_) * (*x as f64 - x_);
        });
        ans /= (data.len() - 1) as f64;

        ans
    }
}
