/*
Primitive Types:
    Integer types: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
    Floats: f32, f64
    Boolean: bool
    Character: char
    Tuples
    Arrays
*/

pub fn run(){
    // default i32
    let x = 1;

    // default f64
    let y = 2.5;
    
    // explicit typing
    let z: i64 = 69696969;

    // max size of var
    println!("Max i32: {}", std::i32::MAX);

    // boolean
    let is_active = true;
    
    println!("{:?}", (x, y, z, is_active));
    
}