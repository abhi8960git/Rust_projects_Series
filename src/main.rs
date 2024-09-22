use std::fmt::format;

pub mod helpers;

fn main() {
    println!("Hello, world!");
    // coercion();
    let Result = helpers::get_full_name("abhid", "shekd");
    println!("Hello from {0}", Result);
}
// type coercion type conversion
// by default the rust var are immutable


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

