fn main() {
    let number:i32 = 300;
    let size = std::mem::size_of_val(&number);
    println!("number = {number} - size = {size}");
    
    let float_number:f32 = 2.5;
    let float_size = std::mem::size_of_val(&float_number);
    println!("float number = {float_number} - size = {float_size}");

    let boolean:bool = false;
    let boolean_size = std::mem::size_of_val(&boolean);
    println!("boolean = {boolean} - size = {boolean_size}");

    let letter:char = 'C';
    let letter_size = std::mem::size_of_val(&letter);
    println!("letter = {letter} - size = {letter_size}");
}
