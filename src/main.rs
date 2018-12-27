use std::io;

fn main() {
    println!("Welcome to BPM counter!");
    println!("Press 'space' to count BPM, 'r' to restart or 'q' to quit.");
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).expect("failed to read line");
        println!("your input: {}", input);
    }
}
