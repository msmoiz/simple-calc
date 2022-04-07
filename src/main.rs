use std::{
    env,
    error::Error,
    io::{stdin, stdout, Write},
};

use crate::calculator::Calculator;

mod calculator;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    // Scripting mode, just return result or fail
    if args.len() > 1 {
        let expression = args[1..].join("");
        match Calculator::evaluate(&expression) {
            Ok(result) => println!("{}", result),
            Err(err) => return Err(format!("{}", err).into()),
        }
        return Ok(());
    }

    // Interactive mode, consume input until exit
    let mut history = vec![0];
    println!("Using simple-calc in interactive mode.");
    println!("Use '$?' to access the previous result.");
    println!("Use '${{N}}' to go further back in history.");
    println!("Enter 'exit' to quit.");
    loop {
        let prompt = ">> ";
        print!("{}", prompt);
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin()
            .read_line(&mut input)
            .expect("Unexpected error while reading input");
        input = input.replace(prompt, "");
        input = input.trim().into();
        input = input.replace("$?", &history.last().unwrap().to_string());
        for (i, result) in history.iter().rev().enumerate() {
            input = input.replace(&format!("${}", i), &result.to_string());
        }

        if input == "exit" {
            println!("Thanks for using simple-calc. Peace.");
            return Ok(());
        }

        match Calculator::evaluate(&input) {
            Ok(result) => {
                println!("The result is {}.", result);
                history.push(result);
            }
            Err(err) => println!("Encountered the following error {}.", err),
        }
    }
}
