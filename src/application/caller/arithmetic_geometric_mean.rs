use calculate_lib::*;

pub const DEFOULT: f64 = 1e-10;
const N: u32 = 100000;

pub fn call(data: &[i64], epsilon: f64) -> String {
    let mut answer = String::new();
    
    answer += &format!("arithmetic geometric mean ({}): ", epsilon);
    
    arithmetic_geometric_mean(data, N, epsilon)
        .iter()
        .enumerate()
        .for_each(|(i, x)| answer += &format!("\n    {} {} : {}", data[i], data[i + 1], *x));
    
    answer
}