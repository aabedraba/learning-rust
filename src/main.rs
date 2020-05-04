use std::io::stdin;


fn main() {
    println!("Guess the number game!");
    println!("Please input your number: ");

    let mut number = String::new();

    stdin().read_line(&mut number)
        .expect("Failed to read line.");

    println!("You guessed: {}", number);
}
