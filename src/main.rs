use std::io;
use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use rand::Rng;

fn main() {
    
    loop {

        println!("Select a program:");
        println!("1) Number Guessing Game");
        println!("2) Variable Mutability Demo");
        println!("3) Open RusTexT");

        let mut selection = String::new();

        io::stdin().read_line(&mut selection)
            .expect("Failed to read line");

        // The variable that the result from .parse() is stored to must have a type defined
        // because of the general nature of the parse method.
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match selection {
            1 => number_guessing(),
            2 => variable_mutability(),
            3 => rustext(),
            _ => {
                println!("Please enter a valid option.");
                continue;
            }
        }
    }
}

fn rustext() {
    println!("--------------");
    println!("Welcome to RusTexT!");
    println!("--------------");

    loop {

        println!("Options:");
        println!("1) Create a new document");
        println!("2) Edit an existing document");

        let mut selection = String::new();

        io::stdin().read_line(&mut selection)
            .expect("Failed to read line");

        // The variable that the result from .parse() is stored to must have a type defined
        // because of the general nature of the parse method.
        let selection: u32 = match selection.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match selection {
            1 => create_document().expect("Failed to write document."),
            2 => edit_document().expect("Failed to write document."),
            _ => {
                println!("Please enter a valid option.");
                continue;
            }
        }
    }
}

fn create_document() -> std::io::Result<()> {
    let mut doc = String::new();
    let mut name = String::new();
    println!("Please write your document below:");
    io::stdin().read_line(&mut doc)?;
    println!("Enter file name:");
    io::stdin().read_line(&mut name)?;
    let mut file = File::create(format!("{}.txt", name.trim()))?;
    write!(file, "{}", doc)?;
    Ok(())
}

fn edit_document() -> std::io::Result<()> {
    let mut doc = File::open("text.txt")?;
    let mut buffer = String::new();
    doc.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

fn number_guessing() {
    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        println!("--------------");
        println!("Welcome to the Number Guessing Game!");
        println!("--------------");
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // New variable guess is a shadow of the already declared variable guess.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn variable_mutability() {
    // mut allows the variable x to be mutable elsewhere in the code.
    let mut x = 5;
    println!("The value of x: {}", x);
    x = 6;
    println!("The value of x: {}", x);

    // Constants must be expressions and have a type annotated.
    const MAX_POINTS: u32 = 100_000;
    println!("Here is a constant value: {}", MAX_POINTS);

    // An example of shadowing. Each let statement is technically a new variable.
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Shadowing allows us to effectively change the type of a variable/
    let spaces = "    ";
    let spaces = spaces.len();
    println!("{}", spaces);
}
