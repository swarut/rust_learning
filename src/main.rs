use std::io;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    println! ("The secret number is {}", secret);
    println!("Please guess the number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Fail to read line");

    println!("You guess : {}", guess);
}
