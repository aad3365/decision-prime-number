use std::io;

fn decision_version1(number: &u32) -> bool {
    let mut divisors: Vec<u32> = Vec::new();

    for i in 1..number + 1 {
        if number % i == 0 {
            divisors.push(i);
        }
    }

    divisors.len() == 2
}

fn main() {
    let mut line = String::new();

    println!("please input some positive number");
    io::stdin().read_line(&mut line).expect("something was wrong while getting input.");
    
    let number = line.trim().parse::<u32>().expect("input must be positive integer");
    assert!(number != 1, "number should be bigger than 1");
    
    match decision_version1(&number) {
        true => println!("{} is prime number!", number),
        false => println!("{} is not a prime number!", number)
    }
}