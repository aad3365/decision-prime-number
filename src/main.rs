use std::io;

// with O(n) complexity
fn decision_version1(number: &u32) -> bool {
    let mut divisors: Vec<u32> = Vec::new();

    for i in 1..number + 1 {
        if number % i == 0 {
            divisors.push(i);
        }
    }

    divisors.len() == 2
}

fn decision_version2(number: &u32) -> bool {
    let mut numbers: Vec<u32> = Vec::new();
    let target = f32::sqrt(*number as f32).floor() as u32;

    for i in 1..*number + 1 {
        numbers.push(i);
    }

    let end_value = numbers[numbers.len() - 1 as usize];

    for i in 2..(target + 1) {
        if numbers[i as usize - 1] == 0 {
            continue;
        }

        for multiplier in 1..end_value {
            let target_index = i * multiplier;

            if target_index > end_value {
                break;
            }

            numbers[target_index as usize - 1] = 0;
        }
    }

    numbers[numbers.len() - 1 as usize] != 0
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

    match decision_version2(&number) {
        true => println!("{} is prime number!", number),
        false => println!("{} is not a prime number!", number)
    }
}
