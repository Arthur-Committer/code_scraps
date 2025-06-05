use std::io;

fn get_idx(b:u8) -> u8{
    return b - b'a'
}

fn main() {
    const VOWEL:u32 = 0x104111;
    
    /*
    let mut hashset:u32 = 0;
    let letters = [b'a',b'e',b'i'/*,b'o',b'u'*/];
    let mut letters_bits:[u32;5] = [0;5];
    for i in 0..letters.len(){
        letters_bits[i] = (1 << (get_idx(letters[i])));
    }
    for e in letters_bits.iter(){
        hashset |= e;
        println!("{:032b}",e);
    }  
    println!("{}",hashset);
    */
        
    let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Error reading the entry");

    let entry = entry.trim_end();
    //let mut entry = entry.to_string();
    
    print!("\x1B[2J\x1B[H");

    let first:u32 =  1 << get_idx(entry.as_bytes()[0]);   
    
    //println!("{:032b}",first);
    //println!("{:032b}\n",VOWEL);

    //println!("{}",VOWEL&first != 0);
    
    if VOWEL&first != 0 {
        println!("{}-hay",entry);
    }
    else{
        println!("{}-{}ay",&entry[1..],&entry[0..1]);
    }
    //println!("{}",entry);
    

    
}
