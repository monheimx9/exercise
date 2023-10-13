use rand::Rng;
use std::io;

fn main() {
    println!("Guess the fucking number Mason!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is = {:?}", secret_num);
    println!("If you guess wrong, you now what happen next");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the damn line");
    println!("You guessed {guess}");
}
