struct Person {
    name: String,
    age: u8,
}
impl Person {
    fn description(&self) -> String {
        // TODO: implement this description function
    }
}
fn main() {
    let person = Person {
        name: "Jack".to_string(),
        age: 34,
    };
    print!("{}", person.description());
}

#[test]
fn describe() {
    let person = Person {
        name: "Jill".to_string(),
        age: 35,
    };
    assert_eq!(person.description(), "Jill is 35 years old");
}
