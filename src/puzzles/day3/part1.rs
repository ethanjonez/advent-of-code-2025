use crate::tools::tools;

pub fn run() {
    let input = tools::read_input("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day3/puzzle".to_string());

    let banks: Vec<&str> = input.split("\n").collect();

    let mut summedOptimalJoltage : i32 = 0;

    for bank in banks {
        let bank_digits: Vec<i32> = bank.split("").filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect();
        
        summedOptimalJoltage += find_optimal_joltage(bank_digits);
    }

    println!("3-1: {}", summedOptimalJoltage);
}

pub fn find_optimal_joltage(bank: Vec<i32>) -> i32 {
    let (firstDigit, firstIndex) = find_max_digit(bank[..bank.len()-1].to_vec());

    let (secondDigit, _) = find_max_digit(bank[firstIndex+1..].to_vec());

    let bankJoltage: String = firstDigit.to_string() + &secondDigit.to_string();

    bankJoltage.parse::<i32>().unwrap()
}

pub fn find_max_digit(list: Vec<i32>) -> (i32,usize) {
    let mut maxIndex : usize = 0;
    let mut max: i32 = -1;

    for (index, digit) in list.iter().enumerate() {
        if *digit > max {
            max = digit.clone();
            maxIndex = index;
        }
    }

    (max, maxIndex)
}