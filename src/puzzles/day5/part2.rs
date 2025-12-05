use crate::tools::tools;

pub fn run() {
    let input = tools::read_input_reference("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day5/puzzle");

    let lines: Vec<&str> = input.split("\n").collect();

    let mut ranges: Vec<(i64, i64)> = Vec::new();

    for line in lines {
        if line == "" {
            break;
        }

    ranges.push((
        line.split("-").collect::<Vec<&str>>()[0].parse::<i64>().unwrap(),
        line.split("-").collect::<Vec<&str>>()[1].parse::<i64>().unwrap()
    ));
    }

    let answer = find_fresh_ingredients(&ranges);

    println!("5-2: {}", answer);
}

pub fn find_fresh_ingredients(ranges: &Vec<(i64, i64)>) -> i64 {
    let mut freshIngredients: i64 = 0;

    let mut ingredientOps: Vec<(i64, &str)> = ranges
        .iter()
        .map(|s| {
            return (s.0, "start");
        })
        .chain(ranges.iter().map(|s| {
            return (s.1, "stop");
        }))
        .collect();

    ingredientOps.sort_by(|a, b| a.0.cmp(&b.0));

    let mut concurrency = 0;
    let mut currentCheckpoint = i64::MAX;
    for ingredientOp in ingredientOps {
        if ingredientOp.1 == "start" {
            concurrency += 1;
        }

        if ingredientOp.1 == "stop" {
            concurrency -= 1;
        }

        if concurrency > 0 && ingredientOp.0 < currentCheckpoint {
            currentCheckpoint = ingredientOp.0;
        }

        if concurrency == 0 {
            freshIngredients += (ingredientOp.0 - currentCheckpoint) + 1;
            currentCheckpoint = i64::MAX;
        }
    }

    freshIngredients
}