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

fn conditionals() {
    let age:u8 = 26;
    let with_adult:bool = true;
    let is_adult:bool = age >= 18;
    let minimum_age:bool = age >= 16;

    if is_adult {
        println!("Allowed");
    } else if minimum_age && with_adult {
        println!("Allow with adult");
    } else {
        println!("Not Allowed");
    }

    let condition = if is_adult { "adult" } else { "minor" };
    println!("Status: {condition}");
}

fn loops() {
    let multiplier:u8 = 5;

    let mut counter:u8 = 1;

    while counter <= 10 {
        let result = counter * multiplier;
        println!("{multiplier} x {counter} = {result}");
        counter += 1;
    }

    counter = 1;
    loop {
        let result = counter * multiplier;
        println!("{multiplier} x {counter} = {result}");
        counter += 1;

        if counter > 10 { break };
    }

    for n in 1..11 {
        let result = n * multiplier;
        println!("{multiplier} x {n} = {result}");
    }
}

fn main() {
    secondary();
    shadow();
    let sum = sum(123, 321);
    println!("Sum is {sum}");

    conditionals();
    loops();

    let language = "PHP";
    let purpose = match language {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Unknown",
    };

    println!("{language} purpose is {purpose}");
}
