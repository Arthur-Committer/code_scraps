use std::io;
use std::process::Command;

fn fibo(limit_number: u128) {
    let (mut n1, mut n2): (u128, u128) = (1, 1);
    let mut temp: u128;
    for i in 1..=limit_number {
        if i == 1 || i == 2 {
            print!("{}{}", if i > 1 { " , " } else { " " }, 1);
        } else {
            temp = n1 + n2;
            print!(" , {}", temp);
            n1 = n2;
            n2 = temp;
        }
    }
}
fn main() {
    println!("Enter a positive number to generate the Fibonacci sequence up to the Nth term:");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Error reading the entry");

    let number: u128 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Your entry isn't a number");
            return;
        }
    };
    Command::new("clear")
        .status()
        .expect("Erro ao limpar terminal");
    for _ in 1..=35 {
        print!("    ");
    }
    println!("First {} Fibonacci numbers below:", number);
    print!("[ ");
    fibo(number);
    println!(" ]");
}
