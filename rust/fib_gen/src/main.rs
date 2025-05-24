use std::io;
fn fibo(limit_number: u128) {
    let mut sequence = vec![1, 1, 2];
    let mut temp: u128;
    let mut i: u128 = 0;
    for _ in 0..((limit_number / 3) + 1) {
        for _ in 0..3 {
            i += 1;
            if i > limit_number {
                return;
            };
            print!("|{}", sequence[0]);

            temp = sequence[1] + sequence[2];
            sequence[0] = sequence[1];
            sequence[1] = sequence[2];
            sequence[2] = temp;
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
    print!("[ ");
    fibo(number);
    println!("| ]");
}
