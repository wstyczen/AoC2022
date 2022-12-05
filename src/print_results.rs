use super::day1;
use super::day2;
use super::day3;
use super::day4;
use super::day5;
use super::util;
use util::DayResults;

use std::time::Instant;

fn print_results<T1, T2>(day: &u32, results: &DayResults<T1, T2>, before: &Instant)
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    let run_time: u128 = before.elapsed().as_millis();
    const HEADER_WIDTH: usize = 33;
    const RESULT_WIDTH: usize = HEADER_WIDTH - 8;
    const FOOTNOTE_WIDTH: usize = HEADER_WIDTH - 13;
    // https://en.wikipedia.org/wiki/Box-drawing_character
    println!("┌{:─^HEADER_WIDTH$}┐", format!("╢ {} ╟", day));
    println!("│ Part 1{:>RESULT_WIDTH$} │", results.part_one);
    println!("│ Part 2{:>RESULT_WIDTH$} │", results.part_two);
    println!("│ Run time{:>FOOTNOTE_WIDTH$} ms │", run_time);
    println!("└{:─^HEADER_WIDTH$}┘", "");
}

pub fn print_day_results(day: &u32) {
    assert!((1..25).contains(day));
    let before = Instant::now();

    match day {
        1 => print_results(&day, &day1::get_results(), &before),
        2 => print_results(&day, &day2::get_results(), &before),
        3 => print_results(&day, &day3::get_results(), &before),
        4 => print_results(&day, &day4::get_results(), &before),
        5 => print_results(&day, &day5::get_results(), &before),
        _ => (),
    };
}
