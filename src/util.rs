pub struct DayResults<T1, T2> {
    pub part_one: T1,
    pub part_two: T2,
}

pub fn print_day_results<T1, T2>(day: u32, results: DayResults<T1, T2>)
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    const HEADER_WIDTH: usize = 33;
    const DAY_WIDTH: usize = HEADER_WIDTH - 7;
    println!("┌{:─^HEADER_WIDTH$}┐", format!("| {} |", day));
    println!(" Part 1:{:>DAY_WIDTH$}", results.part_one);
    println!(" Part 2:{:>DAY_WIDTH$}", results.part_two);
}

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum Part {
    PartOne,
    PartTwo,
}
