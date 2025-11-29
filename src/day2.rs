use crate::util::AdventPuzzle;

const DAY: u8 = 2;

pub struct AdventPuzzleDay2;
impl AdventPuzzle for AdventPuzzleDay2 {
    fn day(&self) -> u8 {
        DAY
    }

    fn part1(&self, input: &str) -> u32 {
        todo!()
    }

    fn part2(&self, input: &str) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::{DayPart, read_answer};

    const SAMPLE_INPUT: &str = "";

    #[test]
    fn test_day2_part1_sample() {
        assert_eq!(AdventPuzzleDay2.part1(SAMPLE_INPUT), 0);
    }
    #[test]
    fn test_day2_part1() {
        assert_eq!(
            AdventPuzzleDay2.part1(&AdventPuzzleDay2.input()),
            read_answer(&DAY, &DayPart::Part1)
        );
    }
    #[test]
    fn test_day2_part2_sample() {
        assert_eq!(AdventPuzzleDay2.part2(SAMPLE_INPUT), 0);
    }
    #[test]
    fn test_day2_part2() {
        assert_eq!(
            AdventPuzzleDay2.part2(&AdventPuzzleDay2.input()),
            read_answer(&DAY, &DayPart::Part2)
        );
    }
}
