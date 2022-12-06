#![allow(non_snake_case)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod print_results;
mod util;

fn main() {
    const LAST_DAY: u32 = 6;
    for i in 1..LAST_DAY + 1 {
        print_results::print_day_results(&i);
    }
}
