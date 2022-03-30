use std::io;

fn main() {
    println!("Guess the number!");
    println!("Input ur guess : ");
    // let x immutable
    // let mut x is mutable
    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line.");

    println!("Your guess: {}", guess);
}
