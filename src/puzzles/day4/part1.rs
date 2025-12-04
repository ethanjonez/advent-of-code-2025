use crate::tools::tools;

pub fn run() {
    let input = tools::read_input_reference("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day4/puzzle");

    let lines: Vec<&str> = input.split("\n").collect();

    let mut matrix: Vec<Vec<&str>> = Vec::new();

    for line in lines {
        let row: Vec<&str> = line.split("").filter(|s| !s.is_empty()).collect();
        matrix.push(row);
    }

    let wrappings = find_parcel_wrapping(&matrix);

    println!("4-1: {}", wrappings);
}

pub fn find_parcel_wrapping(matrix: &Vec<Vec<&str>>) -> i32 {
    let mut wrappingCanRmove: i32 = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            let cell = matrix[i][j];

            if cell == "." {
                continue;
            }

            let wrappingAround = get_parcels_surrounding(&matrix, i, j);

            if wrappingAround < 4 {
                wrappingCanRmove += 1;
            }
        }
    }

    wrappingCanRmove
}

pub fn get_parcels_surrounding(matrix: &Vec<Vec<&str>>, i : usize, j : usize) -> i32 {
    let mut wrapping: i32 = 0;

    let iMax = matrix.len();
    let jMax = matrix[0].len();

    if i > 0 && matrix[i-1][j] == "@" {
        wrapping += 1;
    }
    if j > 0 && matrix[i][j-1] == "@" {
        wrapping += 1;
    }
    if i > 0 && j > 0 && matrix[i-1][j-1] == "@" {
        wrapping += 1;
    }
    if i < iMax -1  && matrix[i+1][j] == "@" {
        wrapping += 1;
    }
    if j < jMax-1 && matrix[i][j+1] == "@" {
        wrapping += 1;
    }
    if i > 0 && j < jMax -1 && matrix[i-1][j+1] == "@" {
        wrapping += 1;
    }
    if i < iMax -1 && j > 0 && matrix[i+1][j-1] == "@" {
        wrapping += 1;
    }
    if i < iMax-1 && j < jMax-1 && matrix[i+1][j+1] == "@" {
        wrapping += 1;
    }

    wrapping
}