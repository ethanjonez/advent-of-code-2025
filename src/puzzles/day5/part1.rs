use crate::tools::tools;

pub fn run() {
    let input = tools::read_input_reference("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day5/puzzle");

    let lines: Vec<&str> = input.split("\n").collect();

    let mut ranges: Vec<(i64, i64)> = Vec::new();

    let mut produce: Vec<i64> = Vec::new();

    let mut is_halfway = false;

    for line in lines {
        if line == "" {
            is_halfway = true;
            continue;
        }

        if is_halfway {
            produce.push(line.parse::<i64>().unwrap());
        } else {
            ranges.push((
                line.split("-").collect::<Vec<&str>>()[0].parse::<i64>().unwrap(),
                line.split("-").collect::<Vec<&str>>()[1].parse::<i64>().unwrap()
            ));
        }
    }

    let answer = find_fresh_ingredients(&ranges, &produce);

    println!("5-1: {}", answer);
}

pub fn find_fresh_ingredients(ranges: &Vec<(i64, i64)>, ingredients: &Vec<i64>) -> i64 {
    let mut freshIngredients: i64 = 0;

    for ingredient in ingredients {
        for range in ranges {
            if ingredient >= &range.0 && ingredient <= &range.1 {
                freshIngredients += 1;
                break;
            }
        }
    }

    freshIngredients
}