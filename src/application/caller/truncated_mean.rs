use calculate_lib::*;

pub const DEFOULT: f64 = 0.20;

pub fn call(data: &[i64], p: f64) -> String {
    if p < 0.0 || p > 0.49 {
        format!("truncated mean: {}", "p must be from 0 to 0.49")
    } else {
        format!("truncated mean ({}): {}", p, truncated_mean(data, p))
    }
}

