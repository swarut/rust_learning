use std::io;

fn main() {
    println!("Please guess the number");
    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Fail to read line");

    println!("You guess : {}", guess);
}
