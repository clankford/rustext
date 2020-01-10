use std::io;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    
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
        // New variable selection is a shadow of the already declared variable guess.
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

fn create_document() -> io::Result<()> {
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

fn edit_document() -> io::Result<()> {
    let mut name = String::new();
    
    println!("Please enter the name of the document you want to open:");
    io::stdin().read_line(&mut name)?;
    println!("Opening {}...", name.trim());
    let buffer = match open_document(name) {
        Ok(buffer) => buffer,
        // TODO: Come back to this error handling, it doesn't feel correct.
        // - May want to use a custom error type to have a more standad way
        //   of displaying useful error messages to the user.
        Err(_) => {
            // Convert from &str to String
            "File does not exist.".to_owned()
        },
    };
    println!("{}", buffer);
    Ok(())
}

fn open_document(name: String) -> Result<String, io::Error> {
    let mut doc = File::open(format!("{}.txt", name.trim()))?;
    let mut buffer = String::new();
    doc.read_to_string(&mut buffer)?;
    Ok(buffer)
}

