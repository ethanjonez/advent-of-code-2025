use crate::tools::tools;

pub fn run() {
    let input = tools::read_input("/Users/ejones/Documents/GitHub/advent-of-code-2025/src/puzzles/day1/puzzle_part1".to_string());

    let lines: Vec<&str> = input.lines().collect();

    let mut dial = Dial::new();

    for command in &lines {
        dial.rotate(command);
    }

    println!("1-2: {}", dial.times_at_zero);
}

struct Dial {
    pub position: i32,
    pub times_at_zero: i32,
}

impl Dial {
    pub fn new() -> Self {
        Dial { position: 50, times_at_zero: 0 }
    }

    pub fn rotate(&mut self, command: &str) {
        let direction = &command[0..1];
        let distance_string = &command[1..];

        let distance: i32 = distance_string.parse().expect("Not a valid number");


        for i in 0..distance {
            if direction == "R" {
                self.position += 1;
            }

            if direction == "L" {
                self.position -= 1;
            }

            self.position %= 100;

            if self.position == 0 {
                self.times_at_zero += 1;
            }
        }

        if self.position < 0 {
            self.position += 100;
        }
    }


}