pub mod complex;
pub mod fraction;
pub mod polynomial;
pub mod term;

pub fn sqrt(x: f32) -> f32 {
    if x < 0.0 {
        panic!("Cannot compute the square root of a negative number");
    }

    if x == 0.0 {
        return 0.0;
    }

    let mut guess = if x >= 1.0 { x / 2.0 } else { 1.0 };

    // Divide by x to use relative tolerance to prevent infinite loop
    while (guess * guess - x).abs() / x > 1e-6 {
        guess = (guess + x / guess) / 2.0;
    }

    guess
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sqrt_test() {
        let eps = 1e-5;
        assert!((sqrt(0f32) - 0f32).abs() < eps);
        assert!((sqrt(4f32) - 2f32).abs() < eps);
        assert!((sqrt(9f32) - 3f32).abs() < eps);
    }
}
