use crate::sort;

pub fn moda_mean(data: &[i64]) -> f64 {
    let mut data_copy = data.to_vec();
    let mut max_len: u64 = 0;
    
    let mut num_max: i64 = 0;
    let mut sum_max: i64 = 0;
    
    let mut i_minus_f;
    let mut f: usize = 0;

    sort::sort_shell(&mut data_copy);
    for i in 0..data_copy.len() {
        if data_copy[i] != data_copy[f] {
            i_minus_f = (i - f).try_into().unwrap();
            if max_len < i_minus_f {
                max_len = i_minus_f;
                sum_max = data_copy[f];
                num_max = 1;
            } else if max_len == i_minus_f {
                sum_max += data_copy[f];
                num_max += 1;
            }
            f = i;
        }
    }
    i_minus_f = (data.len() - f).try_into().unwrap();
    if max_len < i_minus_f {
        sum_max = data_copy[f];
        num_max = 1;
    } else if max_len == i_minus_f {
        sum_max += data_copy[f];
        num_max += 1;
    }
    

    sum_max as f64 / num_max as f64
}
