pub struct Person {
    first_name: String,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
}
fn new_person() -> Person {
    let p1 = Person {
        first_name: "Abhishek".to_string(),
        last_name: "Kumar".to_string(),
        birth_year: 9094,
        birth_month: 8,
    };
    return p1;
}

pub fn test_new_person() {
    let myperson = new_person();
    println!(
        "First Name: {0}, Last Name: {1}",
        myperson.first_name, myperson.last_name
    );
}
