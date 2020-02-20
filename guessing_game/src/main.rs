use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guessing game!!");
    // Generating random number
    let secret_number = rand::thread_rng().gen_range(1, 11);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Type a new number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed reading the number");

        //  Cast the variable into unsigned 32 bit number
        // let guess: u32 = guess.trim().parse()
        //     .expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid number!");
                continue;
            } 
        };


        println!("You guessed {}", guess);
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
