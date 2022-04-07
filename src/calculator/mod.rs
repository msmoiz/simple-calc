mod types;

use std::collections::HashMap;

use self::types::{Error, Operator, Result, Token};

pub struct Calculator;

impl Calculator {
    /// Evalutes an input string containing a mathematical expression in
    /// infix notation. Returns the result of the computation or an error
    /// in case of tokenization, validation, or computation failures.
    /// It is capable of handling expressions that contain the following elements;
    /// all other elements are unsupported at this time:
    /// * Integers
    /// * Operands (binary): +, -, *, /
    /// * Parentheses
    pub fn evaluate(input: &str) -> Result<isize> {
        let infix_expression = Calculator::tokenize_expression(input)?;
        Calculator::validate_expression(&infix_expression)?;
        let postfix_expression = Calculator::marshal_infix_expression_to_postfix(infix_expression)?;
        let result = Calculator::evaluate_postfix_expression(postfix_expression)?;
        Ok(result)
    }

    /// Takes an input string and tokenizes it into a mathematical expression
    /// composed of operators and operands. This function does validate the
    /// characters passed as input, but it does not validate the resultant expression.
    fn tokenize_expression(input: &str) -> Result<Vec<Token>> {
        let mut output: Vec<Token> = vec![];
        let mut chars = input.chars().enumerate().peekable();
        while let Some((i, char)) = chars.next() {
            let base_10 = 10;
            if char.is_digit(base_10) {
                let mut num = String::from(char);
                while let Some((_, char)) = chars.peek() {
                    if char.is_digit(base_10) {
                        num.push(chars.next().unwrap().1); // Guaranteed to exist because of while let condition
                    } else {
                        break;
                    }
                }
                output.push(Token::Operand(num.parse().unwrap())); // Guaranteed to succeed because of if condition
            } else {
                match char {
                    '+' => output.push(Token::Operator(Operator::Add)),
                    '-' => output.push(Token::Operator(Operator::Subtract)),
                    '*' => output.push(Token::Operator(Operator::Multiply)),
                    '/' => output.push(Token::Operator(Operator::Divide)),
                    '(' => output.push(Token::LeftParen),
                    ')' => output.push(Token::RightParen),
                    ' ' => continue,
                    _ => return Err(Error::InvalidCharacter(char, i)),
                }
            }
        }
        Ok(output)
    }

    /// This function conducts basic validations on the input expression,
    /// like making sure that operators have operands on both sides and
    /// making sure that there are no consecutive operands. It also checks
    /// for zero-length expression. It does not handle parentheses matching validation.  
    fn validate_expression(expression: &[Token]) -> Result<()> {
        if expression.is_empty() {
            return Err(Error::ZeroLengthExpression);
        }
        let mut tokens = expression.iter().peekable();
        let mut previous: Option<&Token> = None;
        while let Some(token) = tokens.next() {
            let next = tokens.peek();
            match token {
                Token::Operand(operand) => {
                    if let Some(Token::Operand(next_operand)) = next {
                        return Err(Error::InvalidExpression(format!(
                            "consecutive operands {:?}, {:?}",
                            operand, next_operand
                        )));
                    }
                }
                Token::Operator(operator) => {
                    if previous.is_none() {
                        return Err(Error::InvalidExpression(format!(
                            "operator {:?} with no leading operand",
                            operator
                        )));
                    }
                    if next.is_none() {
                        return Err(Error::InvalidExpression(format!(
                            "operator {:?} with no trailing operand",
                            operator
                        )));
                    }
                    match next.unwrap() {
                        Token::Operand(_) => (),
                        Token::LeftParen => (),
                        next_token => {
                            return Err(Error::InvalidExpression(format!(
                                "operator {:?} followed by invalid token {:?}",
                                operator, next_token
                            )));
                        }
                    }
                }
                _ => (),
            }
            previous = Some(token);
        }
        Ok(())
    }

    /// Converts an expression from infix notation (2 + 2) to Reverse Polish Notation (RPN),
    /// otherwise known as postfix notation (2 2 +), using the shunting-yard algorithm:
    /// https://en.wikipedia.org/wiki/Shunting-yard_algorithm. This function performs
    /// no validation beyond parentheses matching; it assumes that the input infix expression is valid.
    fn marshal_infix_expression_to_postfix(expression: Vec<Token>) -> Result<Vec<Token>> {
        let precedence_map = HashMap::from([
            (Operator::Add, 1),
            (Operator::Subtract, 1),
            (Operator::Multiply, 2),
            (Operator::Divide, 2),
        ]); // No good way to make this static without another crate
        let mut output: Vec<Token> = vec![];
        let mut operator_stack: Vec<Token> = vec![];
        'expression_loop: for token in expression {
            match token {
                Token::Operand(operand) => output.push(Token::Operand(operand)),
                Token::LeftParen => operator_stack.push(Token::LeftParen),
                Token::Operator(operator_cur) => {
                    while let Some(operator_prev) = operator_stack.last() {
                        match operator_prev {
                            Token::LeftParen => break,
                            Token::Operator(operator_top) => {
                                if precedence_map[operator_top] > precedence_map[&operator_cur] {
                                    output.push(operator_stack.pop().unwrap()); // Guaranteed to exist because of while let condition
                                } else {
                                    break;
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                    operator_stack.push(Token::Operator(operator_cur));
                }
                Token::RightParen => {
                    while let Some(operator_prev) = operator_stack.last() {
                        match operator_prev {
                            Token::Operator(_) => {
                                output.push(operator_stack.pop().unwrap()); // Guaranteed to exist because of while let condition
                            }
                            Token::LeftParen => {
                                operator_stack.pop().unwrap(); // Guaranteed to exist because of while let condition
                                continue 'expression_loop;
                            }
                            _ => unreachable!(),
                        }
                    }
                    return Err(Error::MismatchedParentheses);
                }
            }
        }
        while let Some(operator) = operator_stack.pop() {
            if operator.eq(&Token::LeftParen) {
                return Err(Error::MismatchedParentheses);
            }
            output.push(operator);
        }
        Ok(output)
    }

    /// Evalutes an expression ordered using Reverse Polish Notation (RPN),
    /// otherwise known as postfix notation. The algorithm proceeds by pushing
    /// each encountered operand onto a stack, popping the last two off and
    /// calculating a result for each operator encountered, and then pushing
    /// the result back on to the stack. Assuming an input with `n` operands
    /// and `n - 1` operators, in proper order, the output will be the sole
    /// remaining value on the stack after all tokens have been spent.
    ///
    /// Note: This function assumes that the input expression represents a valid
    /// postfix notation expression. It performs no validation on the input,
    /// and it will panic if the input is invalid (e.g., '+ 3 2`). It will also
    /// panic on a zero-length expression. As such, input should be validated before
    /// being passed to this function.
    fn evaluate_postfix_expression(expression: Vec<Token>) -> Result<isize> {
        let mut operand_stack: Vec<isize> = vec![];
        for token in expression {
            match token {
                Token::Operand(operand) => operand_stack.push(operand),
                Token::Operator(operator) => {
                    let b = operand_stack.pop().unwrap_or_else(|| {
                        panic!("Found no operands for operator {:?}.", &operator)
                    });
                    let a = operand_stack.pop().unwrap_or_else(|| {
                        panic!("Found only one operand for operator {:?}: {}", &operator, b)
                    });
                    let result = match operator {
                        Operator::Add => Calculator::add(a, b),
                        Operator::Subtract => Calculator::subtract(a, b),
                        Operator::Multiply => Calculator::multiply(a, b),
                        Operator::Divide => Calculator::divide(a, b)?,
                    };
                    operand_stack.push(result)
                }
                _ => unreachable!(), // Postfix notation does not use parentheses
            }
        }
        Ok(operand_stack.pop().expect(
            "No result value found. Please make sure the input expression has non-zero length.",
        ))
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

    fn divide(a: isize, b: isize) -> Result<isize> {
        if b == 0 {
            return Err(Error::DivideByZero(a, b));
        }
        Ok(a / b)
    }
}

#[cfg(test)]
mod tests {
    use super::types::*;
    use super::Calculator;

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
    fn evaluation_behaves_correctly() {
        let i1 = "3 + 4 * 2";
        let o1 = 11;

        let i2 = "3 + 4 / 2";
        let o2 = 5;

        assert_eq!(Calculator::evaluate(i1).unwrap(), o1);
        assert_eq!(Calculator::evaluate(i2).unwrap(), o2);
    }

    #[test]
    fn tokenization_behaves_correctly() {
        let i1 = "3 + 4 * 2";
        let e1 = vec![
            Token::Operand(3),
            Token::Operator(Operator::Add),
            Token::Operand(4),
            Token::Operator(Operator::Multiply),
            Token::Operand(2),
        ];

        let i2 = "3 + 4 / 2";
        let e2 = vec![
            Token::Operand(3),
            Token::Operator(Operator::Add),
            Token::Operand(4),
            Token::Operator(Operator::Divide),
            Token::Operand(2),
        ];

        assert_eq!(Calculator::tokenize_expression(i1).unwrap(), e1);
        assert_eq!(Calculator::tokenize_expression(i2).unwrap(), e2);
    }

    #[test]
    fn tokenization_handles_digit_termination_correctly() {
        let i1 = "3+4*2";
        let e1 = vec![
            Token::Operand(3),
            Token::Operator(Operator::Add),
            Token::Operand(4),
            Token::Operator(Operator::Multiply),
            Token::Operand(2),
        ];
        assert_eq!(Calculator::tokenize_expression(i1).unwrap(), e1);
    }

    #[test]
    fn tokenization_fails_on_unrecognized_character() {
        let i1 = "3+4!2";
        assert!(Calculator::tokenize_expression(i1).is_err());
    }

    #[test]
    fn validation_catches_zero_length_expression() {
        let i1 = vec![];
        assert!(Calculator::validate_expression(&i1).is_err());
    }

    #[test]
    fn validation_catches_consecutive_operands() {
        // 2 2
        let i1 = vec![Token::Operand(2), Token::Operand(2)];
        assert!(Calculator::validate_expression(&i1).is_err());
    }

    #[test]
    fn validation_catches_operor_followed_by_right_paren() {
        // 2 + )
        let i1 = vec![
            Token::Operand(2),
            Token::Operator(Operator::Add),
            Token::RightParen,
        ];
        assert!(Calculator::validate_expression(&i1).is_err());
    }

    #[test]
    fn validation_catches_operator_without_trailing_operand() {
        // 2 +
        let i1 = vec![Token::Operand(2), Token::Operator(Operator::Add)];
        assert!(Calculator::validate_expression(&i1).is_err());
    }

    #[test]
    fn validation_catches_operator_without_leading_operand() {
        // + 2
        let i1 = vec![Token::Operator(Operator::Add), Token::Operand(2)];
        assert!(Calculator::validate_expression(&i1).is_err());
    }

    #[test]
    fn validation_catches_consecutive_operators() {
        // 2 + +
        let i1 = vec![
            Token::Operand(2),
            Token::Operator(Operator::Add),
            Token::Operator(Operator::Add),
        ];
        assert!(Calculator::validate_expression(&i1).is_err());
    }

    #[test]
    fn infix_to_postfix_conversion_behaves_correctly() {
        // 3 + 4 * 2
        let e1 = vec![
            Token::Operand(3),
            Token::Operator(Operator::Add),
            Token::Operand(4),
            Token::Operator(Operator::Multiply),
            Token::Operand(2),
        ];

        // 3 4 2 * +
        let r1 = vec![
            Token::Operand(3),
            Token::Operand(4),
            Token::Operand(2),
            Token::Operator(Operator::Multiply),
            Token::Operator(Operator::Add),
        ];

        // (3 + 4) * 2
        let e2 = vec![
            Token::LeftParen,
            Token::Operand(3),
            Token::Operator(Operator::Add),
            Token::Operand(4),
            Token::RightParen,
            Token::Operator(Operator::Multiply),
            Token::Operand(2),
        ];

        // 3 4 + 2 *
        let r2 = vec![
            Token::Operand(3),
            Token::Operand(4),
            Token::Operator(Operator::Add),
            Token::Operand(2),
            Token::Operator(Operator::Multiply),
        ];

        assert_eq!(
            Calculator::marshal_infix_expression_to_postfix(e1).unwrap(),
            r1
        );
        assert_eq!(
            Calculator::marshal_infix_expression_to_postfix(e2).unwrap(),
            r2
        );
    }

    #[test]
    fn infix_to_postfix_conversion_fails_on_mismatched_parentheses() {
        // ( 3 + 4
        let e1 = vec![
            Token::LeftParen,
            Token::Operand(3),
            Token::Operator(Operator::Add),
            Token::Operand(4),
        ];

        // 3 + 4 )
        let e2 = vec![
            Token::Operand(3),
            Token::Operator(Operator::Add),
            Token::Operand(4),
            Token::RightParen,
        ];

        assert!(Calculator::marshal_infix_expression_to_postfix(e1).is_err());
        assert!(Calculator::marshal_infix_expression_to_postfix(e2).is_err());
    }

    #[test]
    fn postfix_expression_evaluation_behaves_correctly() {
        // 3 4 2 * +
        let e1 = vec![
            Token::Operand(3),
            Token::Operand(4),
            Token::Operand(2),
            Token::Operator(Operator::Multiply),
            Token::Operator(Operator::Add),
        ];

        // 3 4 2 / +
        let e2 = vec![
            Token::Operand(3),
            Token::Operand(4),
            Token::Operand(2),
            Token::Operator(Operator::Divide),
            Token::Operator(Operator::Add),
        ];

        assert_eq!(Calculator::evaluate_postfix_expression(e1).unwrap(), 11);
        assert_eq!(Calculator::evaluate_postfix_expression(e2).unwrap(), 5);
    }

    #[test]
    fn postfix_expression_evaluation_handles_single_operand_input_correctly() {
        let e1 = vec![Token::Operand(5)];
        assert_eq!(Calculator::evaluate_postfix_expression(e1).unwrap(), 5);
    }

    #[test]
    #[should_panic]
    fn postfix_expression_evaluation_panics_on_zero_length_input() {
        let e1 = vec![];
        Calculator::evaluate_postfix_expression(e1).unwrap();
    }
}
