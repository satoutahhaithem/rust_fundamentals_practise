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

    let health = if height <= 180 { "good" } else { "unknown" };
    println!("Health: {}", health);

    // shadowing to a different type
    let health = if height < 180 { true } else { false };

    

}
