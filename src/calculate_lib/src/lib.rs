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

    for i in 0..data.len() / 2 {
        let mut a = data[i] as f64;
        let mut b = data[i + 1] as f64;

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

    for i in 0..data.len() / 2 {
        let mut x = data[i] as f64;
        let mut y = data[i + 1] as f64;
        let mut z = 0 as f64;

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