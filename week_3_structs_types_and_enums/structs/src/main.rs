
#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email : String ,
    phone_number : u32,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn say_hello(person : &Person){
    println!("hello {} how are you?",person.full_name());
}
fn main() {
    println!("{:?}", Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email : "John@email.com".to_string(),
        phone_number : 0745755337,
    });

let person1 = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email : "John@email.com".to_string(),
        phone_number : 0745755337,
    };

 println!("{:?}", person1);
say_hello(&person1);
}


