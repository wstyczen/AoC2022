use super::util;
use util::DayResults;
use util::Part;

use regex::Captures;
use regex::Regex;
use std::collections::BTreeMap;
use std::collections::LinkedList;

struct Command {
    number: u32,
    from: u32,
    to: u32,
}

fn get_group_as_u32(captures: &Captures, index: usize) -> u32 {
    captures
        .get(index)
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .unwrap()
}

fn read_commands() -> Vec<Command> {
    const FILE_PATH: &str = ".//input//Day5_commands.txt";
    let file_contents = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut commands: Vec<Command> = Vec::new();
    let command_re = Regex::new(r"(\d+).*(\d+).*(\d+)").unwrap();
    for &line in &file_contents.split("\n").collect::<Vec<&str>>() {
        if line.is_empty() {
            continue;
        }
        let matches = command_re.captures(line).unwrap();
        commands.push(Command {
            number: get_group_as_u32(&matches, 1),
            from: get_group_as_u32(&matches, 2),
            to: get_group_as_u32(&matches, 3),
        });
    }
    return commands;
}

fn read_initial_crates_configuration() -> BTreeMap<u32, LinkedList<char>> {
    const FILE_PATH: &str = ".//input//Day5_crates.txt";
    let file_contents = std::fs::read_to_string(FILE_PATH).unwrap();

    let mut crate_stacks: BTreeMap<u32, LinkedList<char>> = BTreeMap::new();
    for &line in &file_contents.split("\n").collect::<Vec<&str>>() {
        let mut stack_i: u32 = 1;
        let mut line_i: usize = 1;
        while line_i < line.len() {
            let c = &line.chars().nth(line_i).unwrap();
            if !c.is_whitespace() {
                if !crate_stacks.contains_key(&stack_i) {
                    crate_stacks.insert(stack_i, LinkedList::new());
                }
                crate_stacks
                    .get_mut(&stack_i)
                    .unwrap()
                    .push_front(c.clone());
            }
            line_i += 4;
            stack_i += 1;
        }
    }
    crate_stacks
}

fn execute_command(stacks: &mut BTreeMap<u32, LinkedList<char>>, command: &Command, part: &Part) {
    let mut crates_to_be_moved = command.number.clone();
    match part {
        Part::PartOne => {
            while crates_to_be_moved > 0 && !stacks.get(&command.from).unwrap().is_empty() {
                let removed = stacks.get_mut(&command.from).unwrap().pop_back().unwrap();
                stacks.get_mut(&command.to).unwrap().push_back(removed);
                crates_to_be_moved -= 1;
            }
        }
        Part::PartTwo => {
            let from_stack = stacks.get_mut(&command.from).unwrap();
            let split_off_index = if command.number > from_stack.len().try_into().unwrap() {
                0
            } else {
                from_stack.len() - command.number as usize
            };

            let split = &from_stack.split_off(split_off_index);
            stacks.get_mut(&command.to).unwrap().extend(split);
        }
    }
}

fn execute_commands(
    stacks: &mut BTreeMap<u32, LinkedList<char>>,
    commands: &Vec<Command>,
    part: &Part,
) {
    for command in commands {
        execute_command(stacks, command, part);
    }
}

fn get_top_configuration(stacks: &BTreeMap<u32, LinkedList<char>>) -> String {
    let mut top_configuration = String::new();
    for stack in stacks.values() {
        top_configuration.push(stack.back().unwrap().clone());
    }
    top_configuration
}

pub fn get_results() -> DayResults<String, String> {
    let commands = read_commands();
    let crate_stacks = read_initial_crates_configuration();

    let mut stacks_part_1 = crate_stacks.clone();
    execute_commands(&mut stacks_part_1, &commands, &Part::PartOne);

    let mut stacks_part_2 = crate_stacks.clone();
    execute_commands(&mut stacks_part_2, &commands, &Part::PartTwo);

    DayResults {
        part_one: get_top_configuration(&stacks_part_1),
        part_two: get_top_configuration(&stacks_part_2),
    }
}
