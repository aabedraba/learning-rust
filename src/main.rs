use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("--Guessing time!--");

    let mut secretNumer = rand::thread_rng().gen_range(1, 101);
    
    println!("Random number generated. Time to guess your own number!");

    loop {
        println!("Input new number: ");
        let mut guessed = String::new();
        std::io::stdin().read_line(&mut guessed)
            .expect("Fail reading variable");
        
        let guessed: u32 = match guessed.trim().parse() {
            Ok(num) => num, 
            Err(_) => continue,
        };
        
        match guessed.cmp(& secretNumer) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            }
        };
    } 
}