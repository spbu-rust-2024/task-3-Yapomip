pub fn geometric_mean(data: &[i64]) -> f64 {
    let mut ans: f64 = 1.0;

    data.iter().for_each(|x| {
        ans *= *x as f64;
    });
    ans = ans.powf(1.0f64 / data.len() as f64);

    ans
}
