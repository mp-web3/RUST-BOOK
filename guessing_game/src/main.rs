use std::cmp::Ordering;
use std::io;
// First we add the line use rand::Rng;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    
    // The kind of range expression we’re using here takes the form start..=end 
    // and is inclusive on the lower and upper bounds, 
    // so we need to specify 1..=100 to request a number between 1 and 100.
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    // println!("The secret number is: {secret_number}");
    
    loop {
        println!("Please input your guess:");
        
        let mut guess = String::new();
        
        // In full, the let mut guess = String::new(); 
        // line has created a mutable variable that is currently bound to a new, 
        // empty instance of a String. 
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        // when the user inputs a non-number, 
        // let’s make the game ignore a non-number so the user can continue guessing

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }


}

