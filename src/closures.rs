pub fn test_closures(){
    let add = |x, y| x+y;
    let result =add(3,4);

    let print_result = || println!("this result is{}", result);
    print_result();

    // how to mutate object inside a scope 

    struct Person{
        first_name:String,
        last_name:String
    }

    let mut p1 = Person{first_name: "Trever".to_string(), last_name: "Shan".to_string()};
    let mut change_name = || p1.last_name = "Abhis".to_string();
    change_name();
    println!("Name{}", p1.last_name);

}
