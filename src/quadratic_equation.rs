use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum QuadraticEquatationError {
    #[error("коэффициент a не может быть равен 0")]
    EqualToZero,
    #[error("переполнение")]
    Overflow,
}

pub fn solve(a: f64, b: f64, c: f64) -> Result<Vec<f64>, QuadraticEquatationError> {
    if a.abs() < f64::EPSILON {
        return Err(QuadraticEquatationError::EqualToZero);
    }

    let mut result: Vec<f64> = vec![];

    let discriminant = b * b - 4.0 * a * c;

    if discriminant.abs() < f64::EPSILON {
        result.push((-b + discriminant.sqrt()) / (2.0 * a));
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
    fn not_a_quadratic_equation() {
        let err = solve(0.0, 2.0, 1.0).unwrap_err();
        let expected_err = QuadraticEquatationError::EqualToZero;
        assert_eq!(err, expected_err);
    }

    #[test]
    fn not_a_quadratic_equation_with_a_less_then_epsilon() {
        let err = solve(2.21e-16f64, 2.0, 1.0).unwrap_err();
        let expected_err = QuadraticEquatationError::EqualToZero;
        assert_eq!(err, expected_err);
    }
}
