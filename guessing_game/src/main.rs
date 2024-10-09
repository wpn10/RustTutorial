use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    println!("Please input number from 1 to 10");
    let secret_number = rand::thread_rng().gen_range(1,10);
    loop{
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please give a valid number!");
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("smaller guess"),
            Ordering::Greater => println!("larger guess"),
            Ordering::Equal => {
                println!("Perfect guess");
                break;
            },
        }
    }
}
