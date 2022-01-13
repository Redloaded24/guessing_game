use std::io::stdin;
use rand::Rng;
use std::cmp::Ordering;
use colored::*;

fn main(){

    // Generate the random number
    let secreat_number:i32 = rand::thread_rng().gen_range(0..100);
    let mut life:u8 = 3;

    // Initial Message
    println!("{}","Welcome to the Guessing Game".purple());
    println!("{}","The number is between 0 and 100".purple());

    // The Program loop
    while life > 0{
        
        // The input Part
        let mut guess = String::new();
        println!("{} = {}","Life remaining".yellow(), life);

        // STDIN
        stdin().read_line(&mut guess).unwrap();
        // Printing the guess
        println!("{} {}", "The guess is".yellow(), &guess);
    
        // Converting the guess:String to guess:i32, with error handling
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>{
                println!("{}", "Enter a Valid number".red());
                continue;
            },
        };
        // This is written as told in rust book, Also we can use switch case statement though 😊
        match guess.cmp(&secreat_number) {
            Ordering::Equal => {
                println!("{}","You got it, and.. we don't have any prize for it. LOL".green());
                break;
            },
            Ordering::Greater => {
                if guess > 100{
                    println!("{}","Woah this guess is greater than the limit. Did you read the rules ?".magenta())
                }else{
                    println!("{}","Your guess is greater than the answer".cyan());
                    life -= 1;
                }
            }, 
            Ordering::Less => {
                if guess <= 0{
                    println!("{}","Woah this guess is lower than the limit. Did you read the rules ?".magenta())
                }else{
                    println!("{}","Your guess is less than the answer".cyan());
                    life -= 1;
                }
            }
        }
    }

    println!("The Correct Answer {}",)

}

// Todo
// Get input : Done 
// Generating number : Done
// Comparing the number with the input : Done
// Make the program into a loop: Done
// Error Handling: Done
// Adding color: Done
// Debug if any: Done
// Updating the repo: