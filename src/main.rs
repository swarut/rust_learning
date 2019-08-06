use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1, 101);
    println! ("The secret number is {}", secret);
    println!("Please guess the number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Fail to read line");

    println!("You guess : {}", guess);

    let guess: i32 = guess.trim().parse().expect("Please input a number");
    match guess.cmp(&secret) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
