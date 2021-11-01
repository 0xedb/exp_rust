use std::io;

fn main() {
    println!("hello world");

    let mut ans = String::new();
    io::stdin().read_line(&mut ans).expect("failed to read");

    print!("Guessed {}", ans);
}
