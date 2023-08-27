/*
    This file is lib.rs, this allows us to define functions that can be used by other.

    "If a package contains src/main.rs and src/lib.rs, it has two crates: a library and a binary, both with the same name as the package."
    Read more: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
*/

/*
    This function calculates the slope of a line given two points.
    It is also public, so it can be used by other modules.
*/
pub fn calculate_slope(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (y2 - y1) / (x2 - x1)
}
