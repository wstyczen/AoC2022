use std::collections::BinaryHeap;

#[path = "util.rs"]
mod util;

fn get_calories_per_elf(file_contents: &String) -> BinaryHeap<u32> {
    let mut top_calories_per_elf = BinaryHeap::new();
    let mut current_calories: u32 = 0;
    for line in file_contents.split("\n").collect::<Vec<&str>>() {
        if line.is_empty() {
            top_calories_per_elf.push(current_calories);
            current_calories = 0;
            continue;
        }
        current_calories += line.parse::<u32>().unwrap();
    }
    top_calories_per_elf
}

pub fn print_results() {
    const FILE_PATH: &str = ".//input//Day1.txt";
    let file_contents = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut top_calories_per_elf = get_calories_per_elf(&file_contents);

    let top = top_calories_per_elf.peek().unwrap().clone();
    let top_three_sum = {
        let mut sum = 0;
        for _ in 0..3 {
            sum += top_calories_per_elf.pop().unwrap();
        }
        sum
    };
    util::print_day_results(
        1,
        util::DayResults {
            part_one: top,
            part_two: top_three_sum,
        },
    );
}
