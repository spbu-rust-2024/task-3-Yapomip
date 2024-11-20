fn arithmetic_geometric_mean(first: i64, second: i64, n: u32, epsilon: f64) -> f64 {
    let mut a = first as f64;
    let mut b = second as f64;

    for _j in 0..n {
        let save_a = a;
        let save_b = b;

        if (a - b).abs() < epsilon {
            break;
        }

        a = (save_a + save_b) / 2.0;
        b = (save_a * save_b).sqrt();
    }

    a
}

// n = 100
pub fn arithmetic_geometric_mean_for_array(data: &[i64], n: u32, epsilon: f64) -> Vec<f64> {
    let mut ans: Vec<f64> = Vec::with_capacity(data.len() / 2);

    for i in 0..data.len() / 2 {
        ans.push(arithmetic_geometric_mean(
            data[i * 2],
            data[i * 2 + 1],
            n,
            epsilon,
        ));
    }

    ans
}

#[test]
fn test_arithmetic_geometric_mean() {
    let epsilon = 1e-10;

    assert!(
        (arithmetic_geometric_mean(100, 50, 1000, epsilon)
            - arithmetic_geometric_mean(50, 100, 1000, epsilon))
        .abs()
            < epsilon
    );

    assert!(
        (arithmetic_geometric_mean(100, 50, 1000, epsilon)
            - arithmetic_geometric_mean(100, 50, 990, epsilon))
        .abs()
            < epsilon
    );
}

#[test]
fn test_arithmetic_geometric_mean_for_array() {
    let data: [i64; 4] = [100, 150, 255, 465];
    let epsilon = 10e-10;

    assert_eq!(
        arithmetic_geometric_mean_for_array(&data, 1000, epsilon),
        [
            arithmetic_geometric_mean(data[0], data[1], 1000, epsilon),
            arithmetic_geometric_mean(data[2], data[3], 1000, epsilon)
        ]
    );

    assert_eq!(
        arithmetic_geometric_mean_for_array(&data, 900, epsilon),
        [
            arithmetic_geometric_mean(data[0], data[1], 900, epsilon),
            arithmetic_geometric_mean(data[2], data[3], 900, epsilon)
        ]
    );
}
