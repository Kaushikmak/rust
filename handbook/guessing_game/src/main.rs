use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {
    println!("Guess the number in range of 1 to 10 to win");
    loop{
        let x = rand::thread_rng().gen_range(1..=10);
        println!("Enter a number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        println!("you entered: {guess}");
        let guess:u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&x){
            Ordering::Less => println!("small"),
            Ordering::Equal => {
                println!("You won!!");
                break;
            },
            Ordering::Greater => println!("greater"),
        }
    }
}
