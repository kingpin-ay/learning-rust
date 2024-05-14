use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..=100);
   loop {
        println!("Enter a Number: ");
        let mut number = String::new();
        let _ = io::stdin()
            .read_line(&mut number)
            .expect("Failed to readline");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let result = guess_checker(number, secret_number);
        if result == true {
            break;
        }
    }
}


fn guess_checker(guess: u32, secret: u32) -> bool {

    match guess.cmp(&secret) {
        Ordering::Less => {
            println!("Your guess is too small");
            return false;
        }
        Ordering::Greater => {
            println!("Too big");
            return false;
        }
        Ordering::Equal => {
            println!("You win");
            return true;
        }
    }
}
