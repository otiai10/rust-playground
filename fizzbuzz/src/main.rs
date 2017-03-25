use std::env;

fn fizzbuzz(x: i8) {
    for i in 1..(x + 1) {
        match (i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz!!!"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
            _ => println!("{}", i),
        }
    }
}

fn main() {
    match env::args().last().unwrap().parse::<i8>() {
        Ok(x) => fizzbuzz(x),
        Err(err) => println!("数字を指定してください: {}", err),
    }
}
