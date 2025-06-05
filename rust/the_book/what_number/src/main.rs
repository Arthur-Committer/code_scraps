extern crate rand;
use rand::Rng;

use std::cmp::Ordering;
use std::io;

fn main() {
    let limit: u8 = 100;
    let secret_number: u8 = rand::thread_rng().gen_range(1, limit + 1);
    loop {
        println!("guess the number between 1 and {}", limit);
        //println!("{}", secret_number);
        println!("enter your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("error reading the entry");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you say: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too low"),
            Ordering::Equal => {
                println!("you're right!");
                break;
            }
            Ordering::Greater => println!("too high"),
        }
    }
}
