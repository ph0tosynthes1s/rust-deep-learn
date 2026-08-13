use std::io;

fn main() {
    println!("Guess the num!");
    println!("Insert predictable num");
    
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("Error in string reading!");

    println!("Your num: {}", guess)
}
