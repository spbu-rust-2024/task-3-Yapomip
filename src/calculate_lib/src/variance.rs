use crate::arithmetic_mean::arithmetic_mean;

pub fn variance(data: &[i64]) -> f64 {
    let mut ans: f64 = 0.0;
    let x_ = arithmetic_mean(data);

    data.iter().for_each(|x| {
        ans += (*x as f64 - x_) * (*x as f64 - x_);
    });
    ans /= (data.len() - 1) as f64;

    ans
}
