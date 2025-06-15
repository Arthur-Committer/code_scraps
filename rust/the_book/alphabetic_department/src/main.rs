use std::io;
use std::collections::HashMap;

fn main() {
    let mut employers = HashMap::new();
    employers.insert("arthur","boco");
    println!("{:?}",employers.get(&"arthur"));

    for (key,value) in &employers{
        println!("{}: {}",key,value);
    }
    
}
