use std::fmt::format;

pub mod helpers;

fn main() {
    println!("Hello, world!");
    // coercion();
    let Result = helpers::namehelpers::get_full_name("abhid", "shekd");
    println!("Hello from {0}", Result);
    test_if();
}
// type coercion type conversion
// by default the rust var are immutable


fn test_if(){
    let age_to_drive:u8 = 16u8;

    println!("Enter te person's age: ");
    let myinput :&mut String = &mut String::from("");
    std::io::stdin().read_line(myinput).unwrap();
    
  let age:u8= myinput.replace("\n","").parse::<u8>().unwrap();
    if(age >= age_to_drive){
        println!("Issueing driver's license , because they are old enough");
    }
}

fn coercion(){
    let x:f32 = 255.0;
    let y:u8 = x as u8 - 5;
    println!("{:?}", y);
    let my_str :char = 'A';

    let mut first_name:&str= "Trever";
    first_name = "Bob";

    // tuple
    let name:(&str, u8) = ("Happy", 4);

}

