use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    age: u8,
    hobby: String,
    is_married: bool,
    children: u8,
}

struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
    personal: Person
}

impl User{
    fn new(username: String, email: String, uri: String, active: bool, personal: Person) -> User {
        User {
            username,
            email,
            uri,
            active,
            personal,
        }
    }
    fn print_user(&self) {
        println!("-----USER DATA-----");
        println!("Username: {}", self.username);
        println!("Email: {}", self.email);
        println!("Uri: {}", self.uri);
        if self.active {
            println!("Active");
        }else {
            println!("Not active");
        }
        self.personal.print_person();
    }
}

impl Person { 
    fn new(name: String, age: u8, hobby: String, is_married: bool, children: u8) -> Person {
        Person {
            name,
            age,
            hobby,
            is_married,
            children,
        }
    }
    fn print_person(&self) {
        println!("-----PERSONAL DATA-----");
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Hobby: {}", self.hobby);
        if self.is_married {
            println!("Married");
        }else {
            println!("Not married");
        }
        println!("Children: {}", self.children);
    }
    fn get_person_data_as_json(&self) -> String {
        format!("{{\"name\":\"{}\",\"age\":{},\"hobby\":\"{}\",\"is_married\":{},\"children\":{}}}", self.name, self.age, self.hobby, self.is_married, self.children)
    }

    fn get_person_data_as_serde_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}


fn main() {
    
    let person1 = Person::new("Ante".into(), 25, "Programming".into(), true, 4);
    //person1.print_person();

    let person1_user = User::new("anteperisic".into(), "ante@mycompany.com".into(), "anteperisic.com".into(), true, person1);
    //person1_user.print_user();
    println!("{}", person1_user.personal.get_person_data_as_json());
    println!("--------------------------------------");
    println!("{}", person1_user.personal.get_person_data_as_serde_json());
}
