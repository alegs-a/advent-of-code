#![allow(warnings)]
pub fn part_1(input: String) -> String {
    String::new()
}

pub fn part_2(input: String) -> String {
    String::new()
}

struct Cave {
    data: Vec<Vec<bool>>,
    start_x: usize,
    start_y: usize,
}

impl Cave {
    fn new() -> Self {
        let data = Vec::with_capacity(1000);

        let start_x = 500;
        let start_y = 0;
        Self {
            data,
            start_x,
            start_y,
        }
    }
}
