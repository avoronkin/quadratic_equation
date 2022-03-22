use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum QuadraticEquatationError {
    #[error("коэффициент a не может быть равен 0")]
    CoefficientAEqualToZero,
    #[error("коэффициент не может быть равен бесконечности")]
    CoefficientEqualToInfinity,
    #[error("коэффициент не может быть не числом")]
    CoefficientEqualToNan,
}

pub fn solve(a: f64, b: f64, c: f64) -> Result<Vec<f64>, QuadraticEquatationError> {
    if a.is_infinite() || b.is_infinite() || c.is_infinite() {
        return Err(QuadraticEquatationError::CoefficientEqualToInfinity);
    }
    if a.is_nan() || b.is_nan() || c.is_nan() {
        return Err(QuadraticEquatationError::CoefficientEqualToNan);
    }
    if a.abs() < f64::EPSILON {
        return Err(QuadraticEquatationError::CoefficientAEqualToZero);
    }

    let mut result: Vec<f64> = vec![];

    let discriminant = b * b - 4.0 * a * c;

    if discriminant.abs() < f64::EPSILON {
        result.push(-b / (2.0 * a));
    } else if discriminant >= f64::EPSILON {
        result.push((-b + discriminant.sqrt()) / (2.0 * a));
        result.push((-b - discriminant.sqrt()) / (2.0 * a));
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roots_are_imaginary() {
        let roots = solve(1.0, 0.0, 1.0).unwrap();
        let expected_roots = vec![];
        assert_eq!(roots, expected_roots);
    }

    #[test]
    fn roots_are_real() {
        let roots = solve(1.0, 0.0, -1.0).unwrap();
        let expected_roots = vec![1.0, -1.0];
        assert_eq!(roots, expected_roots);
    }

    #[test]
    fn roots_are_equal() {
        let roots = solve(1.0, 2.0, 1.0).unwrap();
        let expected_roots = vec![-1.0];
        assert_eq!(roots, expected_roots);
    }

    #[test]
    fn should_fail_with_coefficient_a_equal_to_zero() {
        let err = solve(0.0, 2.0, 1.0).unwrap_err();
        let expected_err = QuadraticEquatationError::CoefficientAEqualToZero;
        assert_eq!(err, expected_err);
    }

    #[test]
    fn should_fail_with_coefficient_a_less_then_epsilon() {
        let err = solve(2.21e-16f64, 2.0, 1.0).unwrap_err();
        let expected_err = QuadraticEquatationError::CoefficientAEqualToZero;
        assert_eq!(err, expected_err);
    }

    #[test]
    fn should_fail_with_coefficient_equal_to_infinity() {
        let table: Vec<((f64, f64, f64), QuadraticEquatationError)> = vec![
            ((f64::INFINITY, 2.0, 1.0), QuadraticEquatationError::CoefficientEqualToInfinity),
            ((1.0, f64::INFINITY, 1.0), QuadraticEquatationError::CoefficientEqualToInfinity),
            ((1.0, 2.0, f64::INFINITY), QuadraticEquatationError::CoefficientEqualToInfinity),
        ];

        for ((a, b, c), expected_err) in table {
            let err = solve(a, b, c).unwrap_err();
            assert_eq!(err, expected_err);
        }
    }

    #[test]
    fn should_fail_with_coefficient_equal_to_nan() {
        let table: Vec<((f64, f64, f64), QuadraticEquatationError)> = vec![
            ((f64::NAN, 2.0, 1.0), QuadraticEquatationError::CoefficientEqualToNan),
            ((1.0, f64::NAN, 1.0), QuadraticEquatationError::CoefficientEqualToNan),
            ((1.0, 2.0, f64::NAN), QuadraticEquatationError::CoefficientEqualToNan),
        ];

        for ((a, b, c), expected_err) in table {
            let err = solve(a, b, c).unwrap_err();
            assert_eq!(err, expected_err);
        }
    }
}
