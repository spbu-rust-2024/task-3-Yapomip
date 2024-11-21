use calculate_lib::*;

#[test]
fn test_arithmetic_mean() {
    let data: [i64; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    assert!((arithmetic_mean(&data) - 5.5) < 0.00000001);
}

#[test]
fn test_geometric_mean() {
    let data: [i64; 3] = [1, 2, 3];

    assert!((geometric_mean(&data) - ((1.0 + 2.0 + 3.0) as f64).powf(1.0 / 3.0)) < 0.00000001);
}

#[test]
fn test_generalized_mean() {
    let data: [i64; 3] = [1, 2, 3];

    assert!(
        (generalized_mean(&data, 4)
            - (((1.0 * 1.0 * 1.0 * 1.0 + 2.0 * 2.0 * 2.0 * 2.0 + 3.0 * 3.0 * 3.0 * 3.0) / 3.0)
                as f64)
                .powf(1.0 / 4.0))
            < 0.00000001,
        "generalized mean (4)"
    );

    assert!(
        (generalized_mean(&data, 3)
            - (((1.0 * 1.0 * 1.0 + 2.0 * 2.0 * 2.0 + 3.0 * 3.0 * 3.0) / 3.0) as f64)
                .powf(1.0 / 3.0))
            < 0.00000001,
        "generalized mean (3)"
    );

    assert!(
        (generalized_mean(&data, 0) - ((1.0 * 2.0 * 3.0) as f64).powf(1.0 / 3.0)) < 0.00000001,
        "generalized mean (0)"
    );
}

#[test]
fn test_quasi_arithmetic_mean() {
    let data: [i64; 3] = [1, 2, 3];
    let fi_arithmetic_mean = |x: f64| -> f64 { x }; // arithmetic_mean
    let fi_reverse_arithmetic_mean = |x: f64| -> f64 { x }; // arithmetic_mean
    
    let fi_geometric_mean = |x: f64| -> f64 { x.ln() / (3 as f64).ln() }; // geometric_mean
    let fi_reverse_geometric_mean = |x: f64| -> f64 { (3 as f64).powf(x) }; // geometric_mean
    
    let fi_generalized_mean = |x: f64| -> f64 { x * x }; // generalized_mean
    let fi_reverse_generalized_mean = |x: f64| -> f64 { x.sqrt() }; // generalized_mean

    assert_eq!(
        quasi_arithmetic_mean(&data, fi_arithmetic_mean, fi_reverse_arithmetic_mean),
        arithmetic_mean(&data),
        "fi_arithmetic_mean"
    );
    assert!(
        (quasi_arithmetic_mean(&data, fi_geometric_mean, fi_reverse_geometric_mean)
            - geometric_mean(&data)).abs()
            < 0.00000001,
        "fi_geometric_mean"
    );
    assert!(
        (quasi_arithmetic_mean(&data, fi_generalized_mean, fi_reverse_generalized_mean)
            - generalized_mean(&data, 2)).abs()
            < 0.00000001,
        "fi_generalized_mean"
    );
}

#[test]
fn test_truncated_mean() {
    let data: [i64; 6] = [1, 2, 3, 4, 5, 6];
    
    assert_eq!(
        truncated_mean(&data, 1.0 / data.len() as f64),
        arithmetic_mean(&data[1..data.len() - 1]),
        "canceled 2 values"
    );
    assert!(
        (truncated_mean(&data, 2.0 / data.len() as f64)
            - arithmetic_mean(&data[2..data.len() - 2])).abs()
            < 0.00000001,
        "canceled 4 values"
    );
}

#[test]
fn test_winsorizing_mean() {
    let data: [i64; 6] = [1, 2, 3, 4, 5, 6];
    let data_: [i64; 6] = [2, 2, 3, 4, 5, 5];
    
    assert_eq!(
        truncated_mean(&data, 1.0 / data.len() as f64),
        arithmetic_mean(&data_),
        "canceled 2 values"
    );
    assert!(
        (truncated_mean(&data, 2.0 / data.len() as f64)
            - arithmetic_mean(&data[2..data.len() - 2])).abs()
            < 0.00000001,
        "canceled 4 values"
    );
}

#[test]
fn test_median_mean() {
    let data: [i64; 6] = [1, 2, 3, 4, 5, 6];
    
    assert_eq!(
        median_mean(&data),
        7.0 / 2.0,
    );
    
    let data: [i64; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    assert_eq!(
        median_mean(&data),
        4.0
    );
    
    let data: [i64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    
    assert_eq!(
        median_mean(&data),
        9.0 / 2.0
    );
}

#[test]
fn test_moda_mean() {
    let data: [i64; 6] = [1, 1, 2, 2, 2, 3];
    
    assert_eq!(
        moda_mean(&data),
        2.0,
    );
    
    let data: [i64; 7] = [1, 1, 2, 2, 3, 3, 3];
    
    assert_eq!(
        moda_mean(&data),
        3.0
    );
    
    let data: [i64; 8] = [1, 1, 2, 2, 2, 3, 3, 3];
    
    assert_eq!(
        moda_mean(&data),
        5.0 / 2.0
    );
}

#[test]
fn test_average_linear_deviation() {
    let data: [i64; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    assert_eq!(
        average_linear_deviation(&data),
        (3 + 2 + 1 + 0 + 1 + 2 + 3) as f64 / data.len() as f64,
        "data from 1 to 7"
    );
    
    let data: [i64; 7] = [1, 1, 1, 1, 1, 1, 1];
    
    assert_eq!(
        average_linear_deviation(&data),
        0.0,
        "data all 1"
    );
}

#[test]
fn test_average_quadratic_deviation() {
    let data: [i64; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    assert_eq!(
        average_quadratic_deviation(&data),
        ((3 * 3 + 2 * 2 + 1 * 1 + 0 * 0 + 1 * 1 + 2 * 2 + 3 * 3) as f64 / data.len() as f64).sqrt(),
        "data from 1 to 7"
    );
    
    let data: [i64; 7] = [1, 1, 1, 1, 1, 1, 1];
    
    assert_eq!(
        average_quadratic_deviation(&data),
        0.0,
        "data all 1"
    );
}

#[test]
fn test_linear_coefficient_variation() {
    let data: [i64; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    assert_eq!(
        linear_coefficient_variation(&data),
        ((3 + 2 + 1 + 0 + 1 + 2 + 3) as f64 / data.len() as f64 / 4.0),
        "data from 1 to 7"
    );
    
    assert_eq!(
        linear_coefficient_variation(&data),
        average_linear_deviation(&data) / 4.0,
        "data from 1 to 7 compare with linear_coefficient_variation"
    );
    
    let data: [i64; 7] = [1, 1, 1, 1, 1, 1, 1];
    
    assert_eq!(
        linear_coefficient_variation(&data),
        0.0,
        "data all 1"
    );
}

#[test]
fn test_quadratic_coefficient_variation() {
    let data: [i64; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    assert_eq!(
        quadratic_coefficient_variation(&data),
        ((3 * 3 + 2 * 2 + 1 * 1 + 0 * 0 + 1 * 1 + 2 * 2 + 3 * 3) as f64 / data.len() as f64 / 4.0).sqrt(),
        "data from 1 to 7"
    );
    
    assert_eq!(
        quadratic_coefficient_variation(&data),
        average_quadratic_deviation(&data) / 4.0f64.sqrt(),
        "data from 1 to 7 compare with average_quadratic_deviation"
    );
    
    let data: [i64; 7] = [1, 1, 1, 1, 1, 1, 1];
    
    assert_eq!(
        quadratic_coefficient_variation(&data),
        0.0,
        "data all 1"
    );
}

#[test]
fn test_variance() {
    let data: [i64; 7] = [1, 2, 3, 4, 5, 6, 7];
    
    assert_eq!(
        variance(&data),
        (3 * 3 + 2 * 2 + 1 * 1 + 0 * 0 + 1 * 1 + 2 * 2 + 3 * 3) as f64 / (data.len() - 1) as f64,
        "data from 1 to 7"
    );
    
    let data: [i64; 7] = [2, 4, 6, 8, 10, 12, 14];
    
    assert_eq!(
        variance(&data),
        (6 * 6 + 4 * 4 + 2 * 2 + 0 * 0 + 2 * 2 + 4 * 4 + 6 * 6) as f64 / (data.len() - 1) as f64,
        "odd number from 1 to 14 "
    );
    
    let data: [i64; 7] = [1, 1, 1, 1, 1, 1, 1];
    
    assert_eq!(
        variance(&data),
        0.0,
        "data all 1"
    );
}
