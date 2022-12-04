#[derive(PartialEq, Eq)]
#[allow(dead_code)]
pub enum Part {
    PartOne,
    PartTwo,
}

pub struct DayResults<T1, T2> {
    pub part_one: T1,
    pub part_two: T2,
}
