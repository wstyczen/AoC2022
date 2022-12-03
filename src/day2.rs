use std::collections::HashMap;

#[path = "util.rs"]
mod util;
use util::Part;

#[derive(Hash, PartialEq, Eq, Clone)]
enum Shape {
    Null,
    Rock,
    Paper,
    Scissors,
}

#[derive(Hash, PartialEq, Eq)]
enum RoundResult {
    Null,
    Win,
    Loss,
    Draw,
}

pub struct Round {
    selected_shape: Shape,
    opponents_shape: Shape,
}

fn get_shape_from_char(c: &char) -> Shape {
    match c {
        'A' | 'X' => {
            return Shape::Rock;
        }
        'B' | 'Y' => {
            return Shape::Paper;
        }
        'C' | 'Z' => {
            return Shape::Scissors;
        }
        _ => {
            return Shape::Null;
        }
    }
}

pub fn get_round_from_line(line: &str, part: Part) -> Round {
    let opponents_shape = get_shape_from_char(&line.chars().nth(0).unwrap());
    let selected_shape = if part == Part::PartOne {
        get_shape_from_char(&line.chars().nth(2).unwrap())
    } else {
        pick_shape_to_match_result(
            opponents_shape.clone(),
            get_result_from_char(&line.chars().nth(2).unwrap()),
        )
    };
    return Round {
        selected_shape,
        opponents_shape,
    };
}

impl Round {
    fn get_result(&self) -> RoundResult {
        if self.selected_shape == self.opponents_shape {
            return RoundResult::Draw;
        }
        match self.selected_shape {
            Shape::Rock => {
                return if self.opponents_shape == Shape::Scissors {
                    RoundResult::Win
                } else {
                    RoundResult::Loss
                };
            }
            Shape::Paper => {
                return if self.opponents_shape == Shape::Rock {
                    RoundResult::Win
                } else {
                    RoundResult::Loss
                };
            }
            Shape::Scissors => {
                return if matches!(self.opponents_shape, Shape::Paper) {
                    RoundResult::Win
                } else {
                    RoundResult::Loss
                };
            }
            Shape::Null => RoundResult::Null,
        }
    }

    pub fn get_total_score(&self) -> u32 {
        let scores_per_result: HashMap<RoundResult, u32> = HashMap::from([
            (RoundResult::Win, 6),
            (RoundResult::Draw, 3),
            (RoundResult::Loss, 0),
            (RoundResult::Null, 0),
        ]);
        let scores_per_shape: HashMap<Shape, u32> = HashMap::from([
            (Shape::Rock, 1),
            (Shape::Paper, 2),
            (Shape::Scissors, 3),
            (Shape::Null, 0),
        ]);
        scores_per_shape.get(&self.selected_shape).unwrap()
            + scores_per_result.get(&self.get_result()).unwrap()
    }
}

// Part 2
fn get_result_from_char(c: &char) -> RoundResult {
    match c {
        'X' => RoundResult::Loss,
        'Y' => RoundResult::Draw,
        'Z' => RoundResult::Win,
        _ => RoundResult::Null,
    }
}

fn pick_shape_to_match_result(opponents_shape: Shape, result: RoundResult) -> Shape {
    if result == RoundResult::Null {
        return Shape::Null;
    }
    if result == RoundResult::Draw {
        return opponents_shape;
    }
    match opponents_shape {
        Shape::Rock => {
            return if result == RoundResult::Win {
                Shape::Paper
            } else {
                Shape::Scissors
            };
        }
        Shape::Paper => {
            return if result == RoundResult::Win {
                Shape::Scissors
            } else {
                Shape::Rock
            };
        }
        Shape::Scissors => {
            return if result == RoundResult::Win {
                Shape::Rock
            } else {
                Shape::Paper
            };
        }
        Shape::Null => Shape::Null,
    }
}

pub fn print_results() {
    const FILE_PATH: &str = ".//input//Day2.txt";
    let file_contents = std::fs::read_to_string(FILE_PATH).unwrap();

    let lines = file_contents.split("\n").collect::<Vec<&str>>();

    let mut part_1_result: u32 = 0;
    let mut part_2_result: u32 = 0;
    for &line in &lines {
        if line.is_empty() {
            continue;
        }
        part_1_result += get_round_from_line(&line, Part::PartOne).get_total_score();
        part_2_result += get_round_from_line(&line, Part::PartTwo).get_total_score();
    }

    util::print_day_results(
        2,
        util::DayResults {
            part_one: part_1_result,
            part_two: part_2_result,
        },
    );
}
