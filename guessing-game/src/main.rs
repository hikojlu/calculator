use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let rnd_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number program!");
    println!("Input your number");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was {guess}");

        match guess.cmp(&rnd_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
