pub struct Calculator;

impl Calculator {
    fn calculate(expression: Expression) -> Result<isize, String> {
        expression
            .0
            .into_iter()
            .try_fold(0, |x: isize, item: Operation| match item {
                Operation::Add(o) => match o {
                    Operand::Value(y) => Ok(Calculator::add(x, y)),
                    Operand::Expression(e) => {
                        Calculator::calculate(e).map(|y| Calculator::add(x, y))
                    }
                },
                Operation::Subtract(o) => match o {
                    Operand::Value(y) => Ok(Calculator::subtract(x, y)),
                    Operand::Expression(e) => {
                        Calculator::calculate(e).map(|y| Calculator::subtract(x, y))
                    }
                },
                Operation::Multiply(o) => match o {
                    Operand::Value(y) => Ok(Calculator::multiply(x, y)),
                    Operand::Expression(e) => {
                        Calculator::calculate(e).map(|y| Calculator::multiply(x, y))
                    }
                },
                Operation::Divide(o) => match o {
                    Operand::Value(y) => Calculator::divide(x, y),
                    Operand::Expression(e) => {
                        Calculator::calculate(e).and_then(|y| Calculator::divide(x, y))
                    }
                },
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

pub struct Expression(Vec<Operation>);

impl From<Vec<Operation>> for Expression {
    fn from(operations: Vec<Operation>) -> Self {
        Expression(operations)
    }
}

pub enum Operation {
    Add(Operand),
    Subtract(Operand),
    Multiply(Operand),
    Divide(Operand),
}

pub enum Operand {
    Value(isize),
    Expression(Expression),
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    use super::Operand::*;
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
        assert_eq!(
            Calculator::calculate(vec![Add(Value(1)), Add(Value(2))].into()).unwrap(),
            3
        );
        assert_eq!(
            Calculator::calculate(vec![Add(Value(1)), Subtract(Value(2))].into()).unwrap(),
            -1
        );
        assert_eq!(
            Calculator::calculate(vec![Add(Value(8)), Multiply(Value(2))].into()).unwrap(),
            16
        );
        assert_eq!(
            Calculator::calculate(vec![Add(Value(10)), Divide(Value(5))].into()).unwrap(),
            2
        );
    }

    #[test]
    fn calculation_behaves_correctly_with_zero_divisor() {
        assert!(Calculator::calculate(vec![Add(Value(5)), Divide(Value(0))].into()).is_err());
    }

    #[test]
    fn calculation_can_handle_nested_expressions() {
        assert_eq!(
            Calculator::calculate(
                vec![
                    Add(Value(5)),
                    Add(Expression(vec![Add(Value(4)), Divide(Value(2))].into()))
                ]
                .into()
            )
            .unwrap(),
            7
        );
    }
}
