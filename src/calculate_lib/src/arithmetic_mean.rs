pub fn arithmetic_mean(data: &[i64]) -> f64 {
    let mut ans: f64 = 0.0;

    data.iter().for_each(|x| {
        ans += *x as f64;
    });
    ans /= data.len() as f64;

    ans
}
