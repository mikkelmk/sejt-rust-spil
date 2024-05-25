use rand::Rng;
use std::io;

fn main() {
    loop {
        println!("");
        println!("You are in a game, what do you want to do?");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        if input == "win the game" {
            let random_number = rand::thread_rng().gen_range(1..=3);
            if random_number == 1 {
                println!("You won the game!");
                break;
            }
            println!("Didn't work, try again!");
        } else {
            println!("You can't do that!");
        }
    }
}
