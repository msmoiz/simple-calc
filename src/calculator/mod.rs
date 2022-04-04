mod types;

pub struct Calculator;

impl Calculator {
    pub fn evaluate(_input: String) {
        // tokenize input
        // martial tokens into postfix notation expression
        // evaluate postfix notation expression
    }

    /// This 
    fn evaluate_postfix_expression(expression: Vec<Token>) -> Result<isize, String> {
        let mut operand_stack: Vec<isize> = vec![];
        for token in expression {
            match token {
                Token::Operand(operand) => operand_stack.push(operand),
                Token::Operator(operator) => {
                    let b = operand_stack.pop().unwrap(); // @todo: Is there an error condition here?
                    let a = operand_stack.pop().unwrap(); // @todo: Is there an error condition here?
                    let result = match operator {
                        Operator::Add => Calculator::add(a, b),
                        Operator::Subtract => Calculator::subtract(a, b),
                        Operator::Multiply => Calculator::multiply(a, b),
                        Operator::Divide => Calculator::divide(a, b)?,
                    };
                    operand_stack.push(result)
                },
            }
        }
        Ok(operand_stack.pop().unwrap()) // @todo: Is there an error condition here?
    }

    fn add(a: isize, b: isize) -> isize {
        a + b
    }

    fn subtract(a: isize, b: isize) -> isize {
        a - b
    }

    fn multiply(a: isize, b: isize) -> isize {
        a * b
    }

    fn divide(a: isize, b: isize) -> Result<isize, String> {
        if b == 0 {
            return Err(format!("Attempting to divide by zero: {} / {}", a, b));
        }
        Ok(a / b)
    }
}

pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide
}

pub enum Token {
    Operand(isize),
    Operator(Operator)
}

#[cfg(test)]
mod tests {
    use super::Calculator;
    use super::Token;
    use super::Operator;

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
    fn postfix_expression_evaluation_behaves_correctly() {
        let e1 = vec![Token::Operand(3), Token::Operand(4), Token::Operand(2), Token::Operator(Operator::Multiply), Token::Operator(Operator::Add)];
        let e2 = vec![Token::Operand(3), Token::Operand(4), Token::Operand(2), Token::Operator(Operator::Divide), Token::Operator(Operator::Add)];
        assert_eq!(Calculator::evaluate_postfix_expression(e1).unwrap(), 11);
        assert_eq!(Calculator::evaluate_postfix_expression(e2).unwrap(), 5);
    }

    #[test]
    fn postfix_martialing_behaves_correctly() {
        assert!(false);
    }
}
