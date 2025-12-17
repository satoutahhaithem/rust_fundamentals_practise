fn main() {
    let mut x = 1;

    // continue looping until x > 5
    loop {
        println!("x is {}", x);

        x += 1;

        if x > 5 {
            break;
        }
    }
    let mut i = 0;

    while i <= 5 {
        println!("i = {}", i);
        i += 1;
    }


    for j in 1..=5 {
        println!("j = {}", j);
    }
    
}
