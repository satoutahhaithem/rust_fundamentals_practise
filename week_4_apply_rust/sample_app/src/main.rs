use my_library::{add, multiply, Calculator, Operation};

fn main() {
    // Using free functions from the library
    let sum = add(3, 4);
    let product = multiply(3, 4);
    println!("add(3,4) = {}", sum);
    println!("multiply(3,4) = {}", product);

    // Using the Calculator struct and Operation enum
    let calc = Calculator::new();
    println!("Calculator add: {}", calc.add(10, 20));
    println!("Calculator multiply: {}", calc.multiply(6, 7));
    println!("apply Add: {}", calc.apply(Operation::Add, 2, 5));
    println!("apply Multiply: {}", calc.apply(Operation::Multiply, 2, 5));
}
