pub struct Person {
    name: String,
    age: u32,
    is_male: bool,
}

pub fn structs() -> Person {
    let person = Person {    
        name: String::from("John"),
        age: 30,
        is_male: true,
    };
    person
}