use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Hello to the guessing game!");
    let secret_num = rand::thread_rng().gen_range(1..=20);
    loop {
        println!("Please input youre guess here:");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("Youre guess {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
