use std::io;
use rand::Rng;
fn main() {
    let rnd = rand::thread_rng()
        .gen_range(1, 101);
    
    println!("Secret num created! Pls enter yours!");

    let mut line = String::new();

    io::stdin().read_line(&mut line)
        .expect("Error in string reading!");

    let printed_num:i32 = line.trim().parse().expect("Num parsing troubles!");

    println!("Your num: {}\nSecret: {}", printed_num, rnd);

    if printed_num == rnd {
        println!("Our congrats!!!")
    } else {
        println!("U are wrong...")
    }

}
