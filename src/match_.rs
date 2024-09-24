pub fn test_match() {
    let my_age: u16 = 35;

    match my_age {
        35 => {
            println!("My age is 35");
        }
        _ => {
            println!("My age is not 35");
        }
    }
}

// explore patterns
