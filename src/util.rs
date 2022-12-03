pub struct DayResults<T1, T2> {
    pub part_one: T1,
    pub part_two: T2,
}

pub fn print_day_results<T1, T2>(day: u32, results: DayResults<T1, T2>)
where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    const WIDTH: usize = 33;
    const PREFIX_LEN: usize = 7;
    println!("┌{:─^WIDTH$}┐", format!("| {} |", day));
    println!(
        " Part 1:{:>width$}",
        results.part_one,
        width = WIDTH - PREFIX_LEN
    );
    println!(
        " Part 2:{:>width$}",
        results.part_two,
        width = WIDTH - PREFIX_LEN
    );
}

#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum Part {
    PartOne,
    PartTwo,
}
