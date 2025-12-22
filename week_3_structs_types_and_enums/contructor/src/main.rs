#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
// new fct
    fn from_email(email :String , uri : String) -> Self{
        let username = email
            .split('@')
            .next()
            .unwrap_or("user")
            .to_string();
        println!("the new name is (from the new function) {}",username);
        Self {
            username,
            email,
            uri,
            active:true,
        }
        
    }

    fn update_uri (&mut self,new_uri:String ){
        self.uri =new_uri;
        println!("new_uri executed");
    }
}






fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);
    let mut haithem = User::from_email(
    String::from("satoutahhaithem@gmail.com"),
    String::from("satoutahhaithem@github.com"),
);

    println!("to apply the fct from email {:?}",haithem);
    haithem.update_uri(String::from("s.haithem@esi-sba.dz"));
    println!("after applying new uri {:?}",haithem);

}
