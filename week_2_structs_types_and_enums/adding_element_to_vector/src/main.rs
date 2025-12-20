fn insert_at_ends(v: &mut Vec<i32>, value: i32) {
    v.insert(0, value); // insert at beginning
    v.push(value);      // insert at end
}

fn append_vectors(v1: &mut Vec<i32>, v2: &mut Vec<i32>) {
    v1.append(v2);
}


fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    insert_at_ends(&mut v, 99);
    println!("{:?}", v);


     let mut v = vec![1, 2, 3];
    let mut extra = vec![4, 5, 6];

    append_vectors(&mut v, &mut extra);

    println!("{:?}", v);
    println!("{:?}", extra); // extra will be empty
}
