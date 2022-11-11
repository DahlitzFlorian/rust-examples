use std::cmp::Ordering;
use std::io;
use std::io::Write;

use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("{secret_number}");

    loop {
        print!("Enter a number: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
