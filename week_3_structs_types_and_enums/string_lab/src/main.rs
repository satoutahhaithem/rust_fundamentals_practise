fn longest_word(s: &str) -> &str {
    s.split_whitespace()
        .max_by_key(|w| w.len())
        .unwrap_or("")
}

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();
    // Use slicing to get the first three characters of the sentence
    // println!("{}", &sentence[0..=4]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
   // println!("{}", description);

    // iterate over the characters in the sentence
    // for c in sentence.chars() {
    //     match c {
    //         'a' | 'e' | 'i' | 'o' | 'u' => println!("Got a vowel!"),
    //         _ => continue,
    //     }
    // }


    // Count vowels
    // let mut counts = [0usize; 5]; // a, e, i, o, u

    // for c in sentence.chars() {
    //     match c.to_ascii_lowercase() {
    //         'a' => counts[0] += 1,
    //         'e' => counts[1] += 1,
    //         'i' => counts[2] += 1,
    //         'o' => counts[3] += 1,
    //         'u' => counts[4] += 1,
    //         _ => {}
    //     }
    // }

    
    // println!("a: {}", counts[0]);
    // println!("e: {}", counts[1]);
    // println!("i: {}", counts[2]);
    // println!("o: {}", counts[3]);
    // println!("u: {}", counts[4]);

    // Split and collect into a vector
    //let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    //println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    // println!("{}", reversed);
    let lw = longest_word(&sentence);
    println!("Longest word: {}", lw);

}
