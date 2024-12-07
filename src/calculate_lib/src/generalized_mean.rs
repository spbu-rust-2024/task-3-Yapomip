use crate::geometric_mean::geometric_mean;

// d = 3
pub fn generalized_mean(data: &[i64], d: u32) -> f64 {
    let mut ans: f64 = 0.0;

    if d == 0 {
        ans = geometric_mean(data);
    } else {
        data.iter().for_each(|x| {
            ans += x.pow(d) as f64;
        });
        ans /= data.len() as f64;
        ans = ans.powf(1.0f64 / d as f64);
    }

    ans
}
