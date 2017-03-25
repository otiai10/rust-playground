use std::env;

fn main() {
    for argumens in env::args() {
        println!("{}", argumens)
    }
}
