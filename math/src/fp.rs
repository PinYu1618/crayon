pub fn check_f32(v1: f32, v2: f32) {
    let error_margin = f32::EPSILON;

    assert!((v1 - v2).abs() < error_margin)
}

pub fn check_f64(v1: f64, v2: f64) -> bool {
    let error_margin = f64::EPSILON;

    (v1 - v2).abs() < error_margin
}