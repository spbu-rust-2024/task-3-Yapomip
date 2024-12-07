use calculate_lib::*;

pub const DEFOULT: u32 = 3;

pub fn call(data: &[i64], d: u32) -> String {
    if d > 100 {
        format!("generalized mean: {}", "d must be from 1 to 100")
    } else {
        format!("generalized mean ({}): {}", d, generalized_mean(data, d))
    }
}
