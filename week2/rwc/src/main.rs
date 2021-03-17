use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
}

fn f<'a>(s: &'a str, t: &'a str) -> &'a str {
    if s.len() > 5 {
        s
    } else {
        t
    }
}
