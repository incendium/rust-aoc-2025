use crate::util::AdventPuzzle;

const DAY: u8 = 1;

enum Direction {
    LEFT,
    RIGHT,
}

pub struct AdventPuzzleDay1;
impl AdventPuzzle for AdventPuzzleDay1 {
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> u32 {
        let lines = input.lines();

        let mut zero_count: u32 = 0;
        let mut position = 50;
        for line in lines {
            let (direction, offset) = match get_direction_and_offset(line) {
                Ok(value) => value,
                _ => continue,
            };

            position = match direction {
                Direction::LEFT => (position - offset + 100) % 100,
                Direction::RIGHT => (position + offset) % 100,
            };

            if position == 0 {
                zero_count += 1;
            }
        }

        zero_count
    }

    fn part2(&self, input: &str) -> u32 {
        let lines = input.lines();

        let mut zero_count: u32 = 0;
        let mut position = 50;
        for line in lines {
            let (direction, offset) = match get_direction_and_offset(line) {
                Ok(value) => value,
                _ => continue,
            };
            let offset_remainder = offset % 100;
            let new_position = match direction {
                Direction::LEFT => position - offset_remainder,
                Direction::RIGHT => position + offset_remainder,
            };

            zero_count += (offset / 100).cast_unsigned();
            if position != 0 && (new_position <= 0 || new_position >= 100) {
                zero_count += 1;
            }

            position = (new_position + 100) % 100;
        }

        zero_count
    }
}

fn get_direction_and_offset(line: &str) -> Result<(Direction, i32), ()> {
    let mut chars = line.chars();
    let direction = match chars.next().map(|c| c.to_ascii_lowercase()) {
        Some('l') => Direction::LEFT,
        Some('r') => Direction::RIGHT,
        _ => return Err(()),
    };
    let mut offset_str = String::new();
    chars.for_each(|c| {
        offset_str.push(c);
    });
    let offset = match offset_str.parse::<i32>() {
        Ok(value) => value,
        Err(_) => return Err(()),
    };

    Ok((direction, offset))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{DayPart, read_answer};

    const SAMPLE_INPUT: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_day1_part1_sample() {
        assert_eq!(AdventPuzzleDay1.part1(SAMPLE_INPUT), 3);
    }
    #[test]
    fn test_day1_part1() {
        assert_eq!(
            AdventPuzzleDay1.part1(&AdventPuzzleDay1.input()),
            read_answer(&DAY, &DayPart::Part1)
        );
    }
    #[test]
    fn test_day1_part2_sample() {
        assert_eq!(AdventPuzzleDay1.part2(SAMPLE_INPUT), 6);
    }
    #[test]
    fn test_day1_part2() {
        assert_eq!(
            AdventPuzzleDay1.part2(&AdventPuzzleDay1.input()),
            read_answer(&DAY, &DayPart::Part2)
        );
    }
}
