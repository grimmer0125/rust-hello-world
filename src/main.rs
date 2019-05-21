use std::io;

fn main() {
    println!("Hello, world!");

    let mut guess = String::new();
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
    // io::stdin()
    //     .read_line(&mut guess)
    //     .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
