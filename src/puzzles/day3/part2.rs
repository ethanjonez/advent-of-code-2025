use crate::tools::tools;

pub fn run() {
    let input = tools::read_input("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day3/puzzle".to_string());

    let banks: Vec<&str> = input.split("\n").collect();

    let mut summedOptimalJoltage : i64 = 0;

    for bank in banks {
        let bank_digits: Vec<i32> = bank.split("").filter(|s| !s.is_empty()).map(|s| s.parse::<i32>().unwrap()).collect();

        summedOptimalJoltage += find_optimal_joltage(bank_digits);
    }

    println!("3-2: {}", summedOptimalJoltage);
}

pub fn find_optimal_joltage(bank: Vec<i32>) -> i64 {
    let mut bankJoltage: String = String::new();
    let mut nextIndex: usize = 0;

    for i in 0..12 { // assume goes 0,11 for now
        let (nextMaxDigit, nextMaxIndex) = find_max_digit(bank[nextIndex..bank.len()-(11-i)].to_vec());

        nextIndex += nextMaxIndex + 1;

        bankJoltage += &nextMaxDigit.to_string();
    }
    
    bankJoltage.parse::<i64>().unwrap()
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