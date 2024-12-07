use crate::arithmetic_mean::arithmetic_mean;
use crate::sort;

// procent 0.20
pub fn truncated_mean(data: &[i64], procent: f64) -> f64 {
    let l: usize = (data.len() as f64 * procent) as usize;
    let r: usize = (data.len() as f64 * (1.0 - procent)) as usize;
    let mut data_copy = data[l..r].to_vec();

    sort::sort_shell(&mut data_copy);

    arithmetic_mean(&data_copy)
}
