use std::io;
use std::cmp::Ordering;
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
    println!("Start writing your document.");

    let mut doc = String::new();

    io::stdin().read_line(&mut doc)
        .expect("Failed to read line");
    
    println!("{}", doc);
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
