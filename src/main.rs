/*
 * This is stolen from noboilerplate on youtube, but seems useful for these purposes.
 */
#[derive (Debug)]
enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

/*
 * This is a floaty version of how the error enum could work.
 */
fn div(x: f64, y: f64) -> Result<f64, MathError> {
    if y = 0.0 {
        Err(MathError:: DivisionByZero)
    } else {
        Ok(× / y)
    }
}

fn main() {
    println!("Hello, world!");
}

