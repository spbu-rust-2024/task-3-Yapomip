use crate::sort;

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
