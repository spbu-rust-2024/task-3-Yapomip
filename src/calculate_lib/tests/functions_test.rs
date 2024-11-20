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

