//use std::io;
use std::fs;
use std::path::Path;
use std::time::Instant;


fn get_idx(b: u8) -> u8 {
    // subtract the ASCII value of 'a' from b to get a zero‑based index:
    //   'a' as u8 = 97, so 97 - 97 = 0
    //   'b' as u8 = 98, so 98 - 97 = 1
    //   'c' as u8 = 99, so 99 - 97 = 2
    // this maps lowercase letters 'a','b','c',… to indices 0,1,2,… respectively
    // these indices are then used for bit shifts (1 << index) so that each
    // character’s bit ends up in its own unique position within a bitmask
    // for more 'sensetives' cases use b'A' and use bitmask of 64 bits
    b - b'a'
}


/*
fn no_repeat2(entry:&str) -> char{ 
    let mut hash_set_repeat:u32 = 0;
    let mut hash_set_verified:u32 = 0;
    //println!("{:?}",entry.chars());

    for (i,l) in entry.chars().enumerate(){
        hash_set_verified = 0;
        //println!("{},{}",l,i); 

        if hash_set_repeat&(1 << get_idx(l as u8)) != 0{continue;}
        for (j,le) in entry.chars().enumerate(){
            if hash_set_repeat&(1 << get_idx(le as u8)) != 0{continue;}
            if hash_set_verified&(1 << get_idx(le as u8)) != 0{continue};
            if i == j{
                //println!("mesmo elemento");
            }
            else {
                //println!("{} vs {}",l,le);
                hash_set_verified |= 1 << get_idx(le as u8);
                if l == le{
                    hash_set_repeat |= 1 << get_idx(le as u8);
                    break;
                }               
            }

        }
        if hash_set_repeat&(1 << get_idx(l as u8)) == 0{return l}
    }
    //println!("{:032b}",hash_set_repeate);
    '_'
}
*/
/*
fn get_idx(b: u8) -> u8 {
    b - b'a'
}
fn first_unique_char(entry: &str) -> char {
    let mut bitmask: u32 = 0;
    let mut bitmask2: u32 = 0;
    for l in entry.chars() {
        if bitmask2 & (1 << get_idx(l as u8)) != 0 {
            continue;
        }
        if bitmask & (1 << get_idx(l as u8)) != 0 {
            bitmask2 |= 1 << get_idx(l as u8);
        }
        bitmask ^= 1 << get_idx(l as u8);
    }

    for j in entry.chars() {
        if bitmask & (1 << get_idx(j as u8)) != 0 {
            return j;
        }
    }
    '_'
}
*/
fn first_unique_char(entry: &str) -> char {
    // represents character repetition using a bitmask
    let mut bitmask: u32 = 0;
    // also a bitmask,but indicates when a character has appeared more than once
    // so that we do not modify the first bitmask again
    let mut bitmask2: u32 = 0;

    for l in entry.chars() {
        // if this character is already in hash_set2 (i.e., has appeared more than once),
        // skip it
        if bitmask2 & (1 << get_idx(l as u8)) != 0 {
            continue;
        }
        // if this character is already in hash_set (i.e., has appeared exactly once),
        // record its second occurrence in hash_set2 so we won’t process it again
        if bitmask & (1 << get_idx(l as u8)) != 0 {
            bitmask2 |= 1 << get_idx(l as u8);
        }
        // toggle the bit in hash_set:
        //   –> 1 means “appeared exactly once”
        //   –> 0 means “appeared twice or more” (after toggling a second time it becomes 0)
        bitmask ^= 1 << get_idx(l as u8);
    }

    for j in entry.chars() {
        // if this character appears exactly once (its bit is still 1 in hash_set), return it
        if bitmask & (1 << get_idx(j as u8)) != 0 {
            return j;
        }
    }
    // if every character repeats, return the default '_'
    '_'
}


fn main() {
    
    // read the entry of the user
    /*    
    let mut entry = String::new();
        io::stdin()
            .read_line(&mut entry)
            .expect("Error reading the entry");
    let entry = entry.trim_end();
    */
    

    let content = fs::read_to_string(Path::new("data/list.txt"))
        .expect("Error reading the file");
    let content = content.trim();
    let content = content.strip_prefix('[').unwrap_or(content);
    let content = content.strip_suffix(']').unwrap_or(content);

    let initial_moment = Instant::now();
    
    content.split(',')
        .map(|s| s.trim().trim_matches('\''))
        .for_each(|slice| {
            first_unique_char(slice);
        });
    
    let duration = initial_moment.elapsed();

    println!( 
        "Exec time: {:?} seconds",
        duration
    );    

}
