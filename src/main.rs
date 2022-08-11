use std::env;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();
    let uniform = Uniform::from(33..125);

    let mut password_length : u16 = 12;
    let mut password = String::new();

    if args.len() > 1 {
        password_length = args[1].parse().unwrap();
    }

    println!("Password length: {}", password_length);

    for _ in 0 .. password_length {
        let rand = uniform.sample(&mut rng);
        password.push(char::from_u32(rand).unwrap());
    }

    println!("Your newly generated password: {}", password);
}