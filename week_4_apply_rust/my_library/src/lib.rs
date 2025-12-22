pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn multiply(left: u64, right: u64) -> u64 {
    left * right
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Multiply,
}

pub struct Calculator;

impl Calculator {
    pub fn new() -> Self {
        Calculator
    }

    pub fn add(&self, a: u64, b: u64) -> u64 {
        add(a, b)
    }

    pub fn multiply(&self, a: u64, b: u64) -> u64 {
        multiply(a, b)
    }

    pub fn apply(&self, op: Operation, a: u64, b: u64) -> u64 {
        match op {
            Operation::Add => self.add(a, b),
            Operation::Multiply => self.multiply(a, b),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_free_function() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_multiply_free_function() {
        assert_eq!(multiply(2, 3), 6);
    }

    #[test]
    fn test_calculator_methods() {
        let calc = Calculator::new();
        assert_eq!(calc.add(10, 5), 15);
        assert_eq!(calc.multiply(6, 7), 42);
    }

    #[test]
    fn test_apply_operation() {
        let calc = Calculator::new();
        assert_eq!(calc.apply(Operation::Add, 4, 5), 9);
        assert_eq!(calc.apply(Operation::Multiply, 4, 5), 20);
    }
}
