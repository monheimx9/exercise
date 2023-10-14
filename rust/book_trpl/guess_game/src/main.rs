use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the fucking number Mason!");
    let secret_num = rand::thread_rng().gen_range(1..=100);
    //println!("The secret number is = {:?}", secret_num);
    println!("If you guess wrong, you now what happen next");

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the damn line");
        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too smol"),
            Ordering::Greater => println!("Too bigus"),
            Ordering::Equal => {
                println!("YOU WIN MASON FOR FUCK SAKES");
                break;
            }
        }
    }
}
