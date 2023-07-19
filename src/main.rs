use std::io;

fn main() {
    let mut line = String::new();

    println!("please input some positive number");
    io::stdin().read_line(&mut line).expect("something was wrong while getting input.");
    
    let number = line.trim().parse::<u32>().expect("input must be positive integer");
    let mut divisors: Vec<u32> = Vec::new();

    for i in 1..number + 1 {
        if number % i == 0 {
            divisors.push(i);
        }
    }

    if divisors.len() > 2 {
        println!("{} is not a prime number!", number);
    } else {
        println!("{} is prime number!", number);
    }
}