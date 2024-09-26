use std::cell::Cell;
 struct Person <'p> {
    first_name: Cell<&'p str>,
    last_name: String,
    birth_year: u16,
    birth_month: u8,
}
fn new_person() -> Person<'static> {
    let p1 = Person {
        first_name: Cell::from("Trever"),
        last_name: "Kumar".to_string(),
        birth_year: 9094,
        birth_month: 8,
    };
    p1.first_name.set("shivam");
    return p1;
}

pub fn test_new_person() {
    let myperson = new_person();
    println!(
        "First Name: {0}, Last Name: {1}",
        myperson.first_name.get(), myperson.last_name
    );
}


// Custom Struct
#[derive(Debug)]
#[allow(dead_code)]
struct Vehicle{
    manufacturer:String,
    model :String,
    year:u16,
    color:VehicleColor
}

fn new_vehicle()->Vehicle{
    let v1 =  Vehicle{
        manufacturer:"Porshe".to_string(),
        model:"911".to_string(),
        year:9233,
        color:VehicleColor::Green
    };
    return  v1;
}
#[derive(Debug)]
#[allow(dead_code)]
enum VehicleColor {
    Silver,
    Red,
    Green,
    Yellow
}

pub fn new_method_vehicle(){
    let vehicle = new_vehicle();
    println!("{:?}", vehicle);
}

// 

