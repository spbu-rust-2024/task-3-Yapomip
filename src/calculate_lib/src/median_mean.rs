use crate::sort;

pub fn median_mean(data: &[i64]) -> f64 {
    let mut data_copy = data.to_vec();

    sort::sort_shell(&mut data_copy);
    if data_copy.len() % 2 == 1 {
        data_copy[data_copy.len() / 2] as f64
    } else {
        (data_copy[(data_copy.len() - 1) / 2] + data_copy[(data_copy.len() + 1) / 2]) as f64 / 2.0
    }
}
