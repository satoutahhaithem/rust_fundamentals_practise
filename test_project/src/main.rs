fn main() {
    let mut height = 180;
    height= height + 1;
    println!("The height is: {}", height);
    let result = if height > 175 {
        "Tall"
    } else {
        "Short"
    };
    println!("The person is: {}", result);
}
