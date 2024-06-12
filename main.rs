const PI:f32 = 3.14;
static GLOBAL_VARIABLE:u8 = 1;

fn sum(a:i32, b:i32) -> i32 {
    a + b
}

fn shadow() {
    let a:i32 = 77;

    {
        let a:i32 = 60;
        println!("Inside A: {a}");
    }

    println!("Outside A: {a}");
}

fn secondary() {
    println!("PI = {PI}");
    println!("Global Variable = {GLOBAL_VARIABLE}");

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

fn main() {
    secondary();
    shadow();
    let sum = sum(123, 321);
    println!("Sum is {sum}");
}
