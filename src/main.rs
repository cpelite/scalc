use std::io;

//define static strings
static VER: &str = "SCALC | 2024-01-20";
static DEV: &str = "Dev: CPElite / ZlatinaDev";

fn main() {
    println!("Simple Calculator - type ? a short explanation how to use it.");
    let mut opchoose = String::new();
    io::stdin()
        .read_line(&mut opchoose)
        .expect("Failed to read user input!");

    match opchoose.as_str().trim() {
        "addition" => addition(),
        "+" => addition(),
        "subtract" => subtract(),
        "-" => subtract(),
        "/" => divide(),
        "divide" => divide(),
        "*" => multiply(),
        "multiply" => multiply(),
        "?" => info(),
        "exit" => exit(),
        _ => println!("Unrecognized entry!"),

    }
}

fn addition() {
    println!("Please enter the first number:");
    let mut input_line1 = String::new();
    io::stdin()
        .read_line(&mut input_line1)
        .expect("Failed to read input.");

    let il1p: i32 = input_line1.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    println!("Please enter the second number:");
    let mut input_line2 = String::new();
    io::stdin()
        .read_line(&mut input_line2)
        .expect("Failed to read input.");

    let il2p: i32 = input_line2.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    let result_addition = il1p + il2p;
    
    println!("Result: {}", result_addition);
    main();
}

fn subtract() {
    println!("Please enter the first number:");
    let mut input_line1 = String::new();
    io::stdin()
        .read_line(&mut input_line1)
        .expect("Failed to read input.");

        let il1p: i32 = input_line1.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    println!("Please enter the second number:");
    let mut input_line2 = String::new();
    io::stdin()
        .read_line(&mut input_line2)
        .expect("Failed to read input.");

    let il2p: i32 = input_line2.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    let result_subtract = il1p - il2p;
    
    println!("Result: {}", result_subtract);
    main();
}

fn divide() {
    println!("Please enter the first number:");
    let mut input_line1 = String::new();
    io::stdin()
        .read_line(&mut input_line1)
        .expect("Failed to read input.");

        let il1p: i32 = input_line1.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    println!("Please enter the second number:");
    let mut input_line2 = String::new();
    io::stdin()
        .read_line(&mut input_line2)
        .expect("Failed to read input.");

    let il2p: i32 = input_line2.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    let result_divide = il1p / il2p;
    
    println!("Result: {}", result_divide);
    main();
}

fn multiply() {
    println!("Please enter the first number:");
    let mut input_line1 = String::new();
    io::stdin()
        .read_line(&mut input_line1)
        .expect("Failed to read input.");

        let il1p: i32 = input_line1.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    println!("Please enter the second number:");
    let mut input_line2 = String::new();
    io::stdin()
        .read_line(&mut input_line2)
        .expect("Failed to read input.");

    let il2p: i32 = input_line2.trim()
        .parse()
        .expect("Invalid value. Please enter a number.");

    let result_multiply = il1p * il2p;
    
    println!("Result: {}", result_multiply);
    main();
}

fn info() {
    println!("At the given moment this calculator can only perform the four basic operations.");
    println!("You can choose which operation should be run, by entering +, -, * or /.");
    println!("{VER}");
    println!("{DEV}");
    main();
}

fn exit() {
    std::process::exit(0);
}