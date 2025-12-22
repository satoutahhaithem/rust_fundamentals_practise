## Reflection Questions

### 1. What is a struct in Rust, and what purpose does it serve?
A **struct** in Rust is a custom data type that allows you to group related data into a single logical unit. It is commonly used to represent real-world entities by combining multiple fields of different data types under one name.

**Difference from other data types:**
- **Primitive types** (such as `u8`, `i32`, `bool`) store only a single value.
- **Tuples** can store multiple values of different types but do not name their fields.
- **Structs** store multiple values **with named fields**, making the code more readable, self-documenting, and easier to maintain.

---

### 2. Why is the `#[derive(Debug)]` attribute used for the `Person` struct?
The `#[derive(Debug)]` attribute automatically implements the `Debug` trait for the `Person` struct.

**Benefit:**
- It allows instances of the struct to be printed using the `{:?}` or `{:#?}` format specifiers.
- This is especially useful for debugging, as it enables developers to easily inspect the contents of the struct during program execution.

Without this attribute, attempting to print the struct using `println!("{:?}", person);` would result in a compilation error.
