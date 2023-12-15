/*
 * I got this from noboilerplate on youtube. It seems useful for these purposes.
 */
#[derive (Debug)]
enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

/*
 * These are float-y version of how the error enum could work. Getting this to strings will be the goal.
 */
fn div(x: f64, y: f64) -> Result<f64, MathError> {
    if y = 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(× / y)
    }
}

fn sqrt(x: f64) -> Result<f64, MathError> {
    if x < 0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn ln(x: f64) -> Result<f64, MathError> {
    if x <= 0 {
        Err(MathError:NonPositiveLogarithm)
    } else {
        Ok(x.ln())
    }
}

fn check_math(divis: f64) -> Result<f64, MathError> {
    let answer1 = div(1.0, divis)?;
    let answer2 = sqrt(answer1)?;
    let answer3 = ln(answer2)?;
    Ok(answer3)
}

fn main() {
    check_math(1.0).unwrap();   // => 0.0;
    check_math(0.0).unwrap();   // panic: DevisionByZero
    let err = check_math(0.0);  // => Err(DivisionByZero)
}
