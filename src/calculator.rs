pub struct Calculator;

impl Calculator {
    fn calculate(operations: Vec<Operation>) -> Result<isize, String> {
        operations
            .into_iter()
            .try_fold(0, |x: isize, item: Operation| match item {
                Operation::Add(y) => Ok(Calculator::add(x, y)),
                Operation::Subtract(y) => Ok(Calculator::subtract(x, y)),
                Operation::Multiply(y) => Ok(Calculator::multiply(x, y)),
                Operation::Divide(y) => Calculator::divide(x, y),
            })
    }

    fn add(x: isize, y: isize) -> isize {
        x + y
    }

    fn subtract(x: isize, y: isize) -> isize {
        x - y
    }

    fn multiply(x: isize, y: isize) -> isize {
        x * y
    }

    fn divide(x: isize, y: isize) -> Result<isize, String> {
        if y == 0 {
            Err(format!("Attempting to divide by zero: {} / {}", x, y))
        } else {
            Ok(x / y)
        }
    }
}

pub enum Operation {
    Add(isize),
    Subtract(isize),
    Multiply(isize),
    Divide(isize),
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    use super::Operation::*;

    #[test]
    fn addition_behaves_correctly() {
        assert_eq!(Calculator::add(1, 2), 3);
    }

    #[test]
    fn subtraction_behaves_correctly() {
        assert_eq!(Calculator::subtract(3, 2), 1);
    }

    #[test]
    fn multiplication_behaves_correctly() {
        assert_eq!(Calculator::multiply(2, 3), 6);
    }

    #[test]
    fn division_behaves_correctly() {
        assert_eq!(Calculator::divide(6, 3).unwrap(), 2);
    }

    #[test]
    fn division_behaves_correctly_with_zero_divisor() {
        assert!(Calculator::divide(1, 0).is_err());
    }

    #[test]
    fn calculation_behaves_correctly() {
        assert_eq!(Calculator::calculate(vec![Add(1), Add(2)]).unwrap(), 3);
        assert_eq!(
            Calculator::calculate(vec![Add(1), Subtract(2)]).unwrap(),
            -1
        );
        assert_eq!(
            Calculator::calculate(vec![Add(8), Multiply(2)]).unwrap(),
            16
        );
        assert_eq!(Calculator::calculate(vec![Add(10), Divide(5)]).unwrap(), 2);
    }

    #[test]
    fn calculation_behaves_correctly_with_zero_divisor() {
        assert!(Calculator::calculate(vec![Add(5), Divide(0)]).is_err());
    }
}
