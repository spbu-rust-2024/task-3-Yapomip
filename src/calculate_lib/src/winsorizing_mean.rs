use crate::arithmetic_mean::arithmetic_mean;
use crate::sort;

// procent 0.20
pub fn winsorizing_mean(data: &[i64], procent: f64) -> f64 {
    let l: usize = (data.len() as f64 * procent) as usize;
    let r: usize = (data.len() as f64 * (1.0 - procent)) as usize;
    let mut data_copy = data.to_vec();

    sort::sort_shell(&mut data_copy);

    for i in 0..data_copy.len() {
        if i < l {
            data_copy[i] = data_copy[l];
        } else if i > r {
            data_copy[i] = data_copy[r];
        }
    }

    arithmetic_mean(&data_copy)
}
