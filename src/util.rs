#[allow(dead_code)]
/// Round to a given number of decimal places
/// Example: round_decimals(7.125, 2) -> 7.13
pub fn round_decimals(x: f64, n: u32) -> f64 {
    let base = 10i32;
    let mul  = base.pow(n) as f64;
    (x * mul).round() / mul
}
