use rand::distr::Distribution;
use rand::distr::Uniform;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    let secret_number = {
        let range = Uniform::new(1, 101).unwrap();
        let mut rng = rand::rng();
        range.sample(&mut rng)
    };
    loop {
        println!("Input the number");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        println!("You guessed : {guess}, the secret number was : {secret_number}");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("couldn't parse proper number not");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
            Ordering::Greater => println!("Too large"),
        }
    }
}
