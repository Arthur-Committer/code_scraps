use std::io;
use std::process::Command;

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }
    fn contain(&self, other: &Rectangle) -> bool {
        self.length >= other.length && self.width >= other.width
            || self.length >= other.width && self.width >= other.length
    }
}

fn read_number(prompt: &str) -> u32 {
    loop {
        println!("{}", prompt);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Error reading the entry");

        match input.trim().parse::<u32>() {
            Ok(num) => return num,
            Err(_) => {
                println!("Your entry isn't a positive number");
                continue;
            }
        };
    }
}

fn main() {
    let mut numbers: [u32; 4] = [0; 4];
    println!("Type positive numbers less than 2³² - 1");

    for i in 1..=4 {
        let prompt = format!(
            "Enter the rect{} {}",
            if i == 1 || i == 2 { 1 } else { 2 },
            if i % 2 == 0 { "width" } else { "length" }
        );
        numbers[i - 1] = read_number(&prompt);
    }

    let rect1 = Rectangle {
        length: numbers[0],
        width: numbers[1],
    };
    let rect2 = Rectangle {
        length: numbers[2],
        width: numbers[3],
    };

    Command::new("clear")
        .status()
        .expect("Erro ao limpar terminal");

    println!("");
    println!(
        "{:?} {} be inside {:?}",
        rect2,
        if rect1.contain(&rect2) {
            "can"
        } else {
            "cannot"
        },
        rect1
    );
    println!("");
    println!(
        "The area difference between 
{:#?}
        and 
{:#?} 
        is {}",
        rect1,
        rect2,
        rect1.area().abs_diff(rect2.area())
    );
}
