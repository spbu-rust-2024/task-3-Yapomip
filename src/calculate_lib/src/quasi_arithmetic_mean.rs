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
