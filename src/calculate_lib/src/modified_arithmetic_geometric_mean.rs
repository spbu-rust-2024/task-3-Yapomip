// n 100 epsilon 10e-10
pub fn modified_arithmetic_geometric_mean(first: i64, second: i64, n: u32, epsilon: f64) -> f64 {
    let mut x = first as f64;
    let mut y = second as f64;
    let mut z = 0.0;

    for _j in 0..n {
        let save_x = x;
        let save_y = y;
        let save_z = z;
        let sqrt_from = ((save_x - save_z) * (save_y - save_z)).sqrt();

        if (x - y).abs() < epsilon {
            break;
        }

        x = (save_x + save_y) / 2.0;
        y = save_z + sqrt_from;
        z = save_z - sqrt_from;

        println!("{} :\n    {}\n    {}\n    {}", _j, x, y, z);
    }

    x
}

// n = 100 epsilon 10e-10
pub fn modified_arithmetic_geometric_mean_for_array(
    data: &[i64],
    n: u32,
    epsilon: f64,
) -> Vec<f64> {
    let mut ans: Vec<f64> = Vec::with_capacity(data.len() / 2);

    for i in 0..data.len() / 2 {
        ans.push(modified_arithmetic_geometric_mean(
            data[i * 2],
            data[i * 2 + 1],
            n,
            epsilon,
        ));
    }

    ans
}

#[test]
fn test_modified_arithmetic_geometric_mean() {
    let epsilon = 10e-10;
    let a = modified_arithmetic_geometric_mean(100, 150, 900, epsilon);
    let b = modified_arithmetic_geometric_mean(150, 100, 900, epsilon);

    assert!((a - b).abs() < epsilon,);

    assert!(
        (modified_arithmetic_geometric_mean(100, 50, 1000, epsilon)
            - modified_arithmetic_geometric_mean(100, 50, 990, epsilon))
        .abs()
            < epsilon
    );
}

#[test]
fn test_modified_arithmetic_geometric_mean_for_array() {
    let data: [i64; 4] = [100, 150, 255, 465];
    let epsilon = 10e-10;

    assert_eq!(
        modified_arithmetic_geometric_mean_for_array(&data, 1000, epsilon),
        [
            modified_arithmetic_geometric_mean(data[0], data[1], 1000, epsilon),
            modified_arithmetic_geometric_mean(data[2], data[3], 1000, epsilon)
        ]
    );

    assert_eq!(
        modified_arithmetic_geometric_mean_for_array(&data, 900, epsilon),
        [
            modified_arithmetic_geometric_mean(data[0], data[1], 900, epsilon),
            modified_arithmetic_geometric_mean(data[2], data[3], 900, epsilon)
        ]
    );
}
