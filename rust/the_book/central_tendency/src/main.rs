use std::io;
use std::collections::HashMap;

fn main() {
    let mut vector = Vec::new();
    let mut map:HashMap<i128,usize> = HashMap::new();
    let mut total: i128 = 0;

    let mut mode:(i128,usize) = (0,0);

    loop {
        //clean the terminal
        print!("\x1B[2J\x1B[H");

        let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Error reading the entry");
        let number: i128 = match entry.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Your entry isn't a number");
                continue;
            }
        };
        

        vector.push(number);
        for i in (0..vector.len()-1).rev(){
            if vector[i] > vector[i+1]{
                //print!("[{} e {}], {}",vector[i],number,i);
                vector.swap(i,i+1)    
            }
        }

        total += number;
        
        

        /*
        for _ in 0..vector.len(){
            for i in 0..vector.len()-1{
                if vector[i]>vector[i+1]{
                vector.swap(i,i+1);
                }
            }
        }
        
        print!("[ {}",vector[0]);
        for &element in vector[1..].iter(){
            print!(",{}",element);
        }
        println!(" ]");
        
        for &num in vector.iter(){ 
            let count = map.entry(num).or_insert(0);  
            *count += 1;
        }
        */

        println!("{:?}",vector);
        
        *map.entry(number).or_insert(0) += 1;

        if map[&number] >= mode.1{
            mode.1 = map[&number];
            mode.0 = number
        }

        println!("The total of this dataset is {}",total);
        println!("The mean of this dataset is {}",(total/vector.len() as i128)); 
        println!("The median of this dataset is {}",
            if vector.len()%2 ==0 {
                (vector[vector.len()/2]+vector[(vector.len()/2)-1])/2
            } 
            else {vector[vector.len()/2]});
        println!("The mode of this dataset is {} appeared {} times",mode.0,mode.1);
    }
}
