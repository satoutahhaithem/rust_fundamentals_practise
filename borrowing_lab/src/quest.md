Objective: The objective of this lab is to understand the concept of borrowing in Rust. By the end of the lab, you should be able to distinguish between owning and borrowing variables, comprehend the benefits of borrowing, and correctly use borrowed variables within functions.

Instructions:
Open the lab which will place you in the directory with all the relevant files.

Use the included main.rs file in the src directory to explore the code you will be working with.

In your project directory, open a terminal or command prompt.

Run the command `cargo run` to compile and execute the program.

Observe the output in the console. The program demonstrates the concept of ownership and borrowing in Rust by using different functions and variables.

Reflection Questions:
What is the difference between owning a variable and borrowing a variable in Rust? Why is borrowing important for writing safe and efficient code?

In the provided code, what happens to the variables `my_int`, `my_string`, and `my_vec` when they are passed to the respective functions? Why are some variables still accessible after being passed to a function, while others are not?

Challenge Questions:
Modify the code to create a function named `borrow_vec` that borrows the `my_vec` variable instead of taking ownership. Inside the function, print the vector without modifying it. Call this function from the `main` function.

Extend the code by creating a function named `borrow_string` that borrows the `my_string` variable and appends some text to it without taking ownership. Print the modified string inside the function and then print it again in the `main` function to observe the changes.

Remember to test your modifications by running the program and observing the output. Experiment with different scenarios to deepen your understanding of borrowing in Rust.