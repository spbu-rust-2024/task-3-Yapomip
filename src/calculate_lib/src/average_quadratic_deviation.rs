use crate::arithmetic_mean::arithmetic_mean;

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
