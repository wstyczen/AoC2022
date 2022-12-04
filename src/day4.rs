#[path = "util.rs"]
mod util;

use regex::Captures;
use regex::Regex;

struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.low <= other.low && self.high >= other.high
    }
    fn get_no_overlaping(&self, other: &Range) -> u32 {
        if self.low > other.high || self.high < other.low {
            return 0;
        }
        return 1 + std::cmp::min(self.high, other.high) - std::cmp::max(self.low, other.low);
    }
}

fn get_group(captures: &Captures, index: usize) -> u32 {
    captures
        .get(index)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .unwrap()
}

fn parse_range_pair(string: &str) -> (Range, Range) {
    let re = Regex::new(r"^(\d+)\-(\d+),(\d+)\-(\d+)\s*$").unwrap();
    let captures = re.captures(string).unwrap();

    return (
        Range {
            low: get_group(&captures, 1),
            high: get_group(&captures, 2),
        },
        Range {
            low: get_group(&captures, 3),
            high: get_group(&captures, 4),
        },
    );
}

pub fn print_results() {
    const FILE_PATH: &str = ".//input//Day4.txt";
    let file_contents = std::fs::read_to_string(FILE_PATH).unwrap();

    let (mut one_contains_another_counter, mut no_of_overlapping_pairs): (u32, u32) = (0, 0);
    for line in file_contents.split("\n").collect::<Vec<&str>>() {
        if line.is_empty() {
            continue;
        }
        let (first, second) = parse_range_pair(line);
        // Part 1
        if first.contains(&second) || second.contains(&first) {
            one_contains_another_counter += 1;
        }
        // Part 2
        if first.get_no_overlaping(&second) != 0 {
            no_of_overlapping_pairs += 1;
        }
    }

    util::print_day_results(
        4,
        util::DayResults {
            part_one: one_contains_another_counter,
            part_two: no_of_overlapping_pairs,
        },
    );
}
