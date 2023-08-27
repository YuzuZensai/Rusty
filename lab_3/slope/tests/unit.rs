use float_cmp::approx_eq;

// Test float-cmp crate, that computes floating point values with a given epsilon
#[test]
fn test_float() {
    assert!(approx_eq!(f64, 1.0 / 3.0, 0.3333, epsilon = 0.0001));
}

// Test slope calculation
#[test]
fn test_slope_calculation() {
    assert!(approx_eq!(f64, slope::calculate_slope(0.0, 0.0, 1.0, 1.0), 1.0, epsilon = 0.0001));
    assert!(approx_eq!(f64, slope::calculate_slope(0.0, 0.0, 2.0, 3.0), 1.5, epsilon = 0.0001));
    assert!(approx_eq!(f64, slope::calculate_slope(1.0, 2.0, 3.0, 4.0), 1.0, epsilon = 0.0001));
}
