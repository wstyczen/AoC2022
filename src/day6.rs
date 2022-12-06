use super::util;
use util::DayResults;

use std::collections::HashSet;

fn is_marker(datastream: &String, i: &usize, group_size: &usize) -> bool {
    let unique_chars: HashSet<char> = datastream[i - group_size..*i].chars().into_iter().collect();
    unique_chars.len() == *group_size
}

fn get_first_marker_index(datastream: &String, group_size: &usize) -> u32 {
    for i in *group_size..datastream.len() {
        if is_marker(datastream, &i, &group_size) {
            return i as u32;
        }
    }
    return 0;
}

pub fn get_results() -> DayResults<u32, u32> {
    const INPUT_FILE_PATH: &str = ".//input//Day6.txt";
    let file_contents = std::fs::read_to_string(INPUT_FILE_PATH).unwrap();

    const GROUP_SIZE_PART_1: usize = 4;
    const GROUP_SIZE_PART_2: usize = 14;
    DayResults {
        part_one: get_first_marker_index(&file_contents, &GROUP_SIZE_PART_1),
        part_two: get_first_marker_index(&file_contents, &GROUP_SIZE_PART_2),
    }
}
