pub fn run() {
    let name = "Aidan";
    let mut age = 19;
    age = 20;
    println!("Name: {name}", name=name);
    println!("Age: {age}", age=age);

    // define constants
    const ID: i32 = 001;
    println!("ID: {}", ID);
    
    // multiple assignment
    let (my_name, my_age) = ("Aidan", 19);
    println!("Name: {}, Age: {}", my_name, my_age)
}