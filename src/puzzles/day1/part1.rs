use crate::tools::tools;

pub fn run() {
    let input = tools::read_input("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day1/puzzle_part1".to_string());

    let lines: Vec<&str> = input.lines().collect();

    let mut dial = Dial::new();

    let mut times_at_zero = 0;

    for command in &lines {
        dial.rotate(command);

        if dial.position == 0 {
            times_at_zero += 1;
        }
    }

    println!("Times at zero: {}", times_at_zero);
}

struct Dial {
    pub position: i32,
}

impl Dial {
    pub fn new() -> Self {
        Dial { position: 50 }
    }

    pub fn rotate(&mut self, command: &str) {
        let direction = &command[0..1];
        let distance_string = &command[1..];

        let distance: i32 = distance_string.parse().expect("Not a valid number");

        if direction == "R" {
            self.position += distance;
        }

        if direction == "L" {
            self.position -= distance;
        }

        self.position %= 100;

        if self.position < 0 {
            self.position += 100;
        }
    }
}