use std::collections::HashSet;

use super::util;
use util::DayResults;

fn get_common_char(strings: (&str, &str)) -> Option<char> {
    let (first, second): (&str, &str) = strings;
    let first_as_set: HashSet<char> = first.chars().collect();
    second.chars().find(|c| first_as_set.contains(&c)).clone()
}

fn get_common_chars(strings: (&String, &String)) -> String {
    let (first, second): (&String, &String) = strings;
    let first_as_set: HashSet<char> = first.chars().collect();
    return second
        .chars()
        .into_iter()
        .filter(|c| first_as_set.contains(c))
        .collect::<String>();
}

fn get_common_between_all(strings: &Vec<String>) -> Option<char> {
    if strings.len() <= 1 {
        return Option::None;
    }

    let mut common = String::from(strings.first().unwrap().clone());
    for i in 1..strings.len() {
        common = get_common_chars((strings.get(i).unwrap(), &common));
    }

    if common.is_empty() {
        Option::None
    } else {
        common.chars().nth(0)
    }
}

fn get_priority(c: char) -> u32 {
    if c.is_lowercase() {
        return u32::from(1 + c as u8 - 'a' as u8);
    }
    u32::from(27 + c as u8 - 'A' as u8)
}

pub fn get_results() -> DayResults<u32, u32> {
    const FILE_PATH: &str = ".//input//Day3.txt";
    let file_contents = std::fs::read_to_string(FILE_PATH).unwrap();

    let (mut priority_sum_halves, mut priority_sum_threes): (u32, u32) = (0, 0);
    let mut three_lines = Vec::new();
    for line in file_contents.split("\n").collect::<Vec<&str>>() {
        // Part 1
        let common_char_halves = get_common_char(line.split_at(line.len() / 2));
        if common_char_halves.is_some() {
            priority_sum_halves += get_priority(common_char_halves.unwrap());
        }
        // Part 2
        three_lines.push(String::from(line.clone()));
        if three_lines.len() == 3 {
            let common_char_threes = get_common_between_all(&three_lines);
            if common_char_threes.is_some() {
                priority_sum_threes += get_priority(common_char_threes.unwrap());
            }
            three_lines.clear();
        }
    }

    DayResults {
        part_one: priority_sum_halves,
        part_two: priority_sum_threes,
    }
}
