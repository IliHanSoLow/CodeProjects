use rand::Rng;
use std::cmp::Ordering;
use std::io;
use color_print::cprintln;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    let mut tries = 0;

    loop {
        tries += 1;
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Greater => cprintln!("<red>To large</red>"),
            Ordering::Less => cprintln!("<red>To small</red>"),
            Ordering::Equal => {cprintln!("<green>Wow u did it it only took you {} tries 😒 amateur</green>", tries); break;}
        }
        println!();
    }
    // match guess.cmp(&secret_number) {
    // Ordering::Equal => print!("Wow you are right the secret number is {}", secret_number),
    // _ => println!("Your shouldnt try Lotto you lost a 1/100 chance. You had chosen: {} it was {}", guess, secret_number)

}