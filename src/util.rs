use std::fs;

/// Trait for basic puzzle structure.
pub(crate) trait AdventPuzzle {
    fn day(&self) -> u8;

    fn input(&self) -> String {
        read_input(&self.day())
    }

    fn part1(&self, input: &str) -> u32;
    fn solve_part1(&self) {
        println!("Day {}, Part 1: {}", self.day(), self.part1(&self.input()));
    }

    fn part2(&self, input: &str) -> u32;
    fn solve_part2(&self) {
        println!("Day {}, Part 2: {}", self.day(), self.part2(&self.input()));
    }

    fn solve(&self) {
        self.solve_part1();
        self.solve_part2();
    }
}

pub(crate) enum DayPart {
    Part1,
    Part2,
}
impl DayPart {
    fn to_num(&self) -> u8 {
        match self {
            DayPart::Part1 => 1,
            DayPart::Part2 => 2,
        }
    }
}

/// Read the puzzle input from the data directory.
fn read_input(day: &u8) -> String {
    fs::read_to_string(format!("data/day{}.input.txt", day))
        .map(|s| s.trim().to_string())
        .expect("Error reading input file")
}

/// Read the answer from the data directory for unit tests.
#[allow(dead_code)]
pub(crate) fn read_answer(day: &u8, part: &DayPart) -> u32 {
    fs::read_to_string(format!("data/day{}.part{}.answer.txt", day, part.to_num()))
        .expect("Error reading answer file")
        .trim()
        .parse::<u32>()
        .expect("Error reading answer file")
}
