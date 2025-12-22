// Enum representing different shapes. Variants carry the
// data needed to compute their areas.
enum Shape {
    Circle(f64),        // radius
    Square(f64),        // side length
    Triangle(f64, f64), // base, height
}

impl Shape {
    // Compute the area for each shape variant.
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Square(s) => s * s,
            Shape::Triangle(base, height) => 0.5 * base * height,
        }
    }

    // Return a short description useful for printing.
    fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("Circle (radius: {:.2})", r),
            Shape::Square(s) => format!("Square (side: {:.2})", s),
            Shape::Triangle(b, h) => format!("Triangle (base: {:.2}, height: {:.2})", b, h),
        }
    }
}

// Return the shape with the largest area, or `None` if the slice is empty.
fn largest_shape(shapes: &[Shape]) -> Option<&Shape> {
    shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap_or(std::cmp::Ordering::Equal))
}

fn main() {
    // Include a Triangle variant in the vector
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Square(3.0),
        Shape::Triangle(20.0, 10.0), // area = 100.0
    ];

    // Compute total area using the `area()` method
    let total_area: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("Total area: {:.2} sq. units", total_area);

    // Show each shape and its area
    println!("\nDetails per shape:");
    for s in &shapes {
        println!("{} => area: {:.2}", s.describe(), s.area());
    }

    // Find and print the largest shape
    match largest_shape(&shapes) {
        Some(s) => println!("\nLargest shape: {} with area {:.2}", s.describe(), s.area()),
        None => println!("No shapes provided."),
    }
}
