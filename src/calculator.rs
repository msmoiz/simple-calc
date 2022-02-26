pub struct Calculator;

impl Calculator {
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
      return Err(format!("Attempting to divide by zero: {}/{}", x, y));
    }
    Ok(x / y)
  }
}

#[cfg(test)]
mod tests {
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
}