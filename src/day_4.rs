use std::fs;

pub fn day_four() {
    let raw_input = fs::read_to_string("input/4.txt").expect("Error reading input file");

    part_one(raw_input.clone());
    part_two(raw_input.clone());
}

fn part_one(input: String) {
    let mut count = 0;

    for line in input.lines() {
        let pair = Pair::new(line);
        if pair.one_contains_other() {
            count += 1;
        }
    }
    println!("PART 1: Total number of pairs where one contains the other is: {count}");
}

fn part_two(input: String) {
    let mut count = 0;

    for line in input.lines() {
        let pair = Pair::new(line);
        if pair.overlaps() {
            count += 1;
        }
    }
    println!("PART 2: The total numer of pairs that overlap at all is: {count}");
}

#[derive(Debug, Clone, Copy)]
struct Pair {
    first: Range,
    second: Range,
}

impl Pair {
    fn new(input: &str) -> Pair {
        let halves: Vec<&str> = input.split(',').collect();

        let first_half: Vec<&str> = halves[0].split('-').collect();
        let second_half: Vec<&str> = halves[1].split('-').collect();

        let first = Range {
            lower: first_half[0].parse::<i32>().unwrap(),
            upper: first_half[1].parse::<i32>().unwrap(),
        };
        let second = Range {
            lower: second_half[0].parse::<i32>().unwrap(),
            upper: second_half[1].parse::<i32>().unwrap(),
        };

        Pair { first, second }
    }

    fn one_contains_other(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn overlaps(&self) -> bool {
        self.one_contains_other()
            || self.first.lower < self.second.lower && self.first.upper >= self.second.lower
            || self.second.lower < self.first.lower && self.second.upper >= self.first.lower
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    lower: i32,
    upper: i32,
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.lower <= other.lower && self.upper >= other.upper
    }
}

#[cfg(test)]
mod tests {
    use super::{Pair, Range};

    #[test]
    fn test_contains() {
        let small = Range { lower: 1, upper: 3 };
        let big = Range { lower: 1, upper: 4 };
        assert_eq!(big.contains(&small), true);
    }

    #[test]
    fn test_overlaps() {
        let first = Range { lower: 1, upper: 3 };
        let second = Range { lower: 3, upper: 4 };
        let good_pair = Pair { first, second };
        let bad_first = Range { lower: 2, upper: 4 };
        let bad_second = Range { lower: 5, upper: 5 };
        let bad_pair = Pair {
            first: bad_first,
            second: bad_second,
        };
        assert_eq!(good_pair.overlaps(), true, "good pair");
        assert_eq!(bad_pair.overlaps(), false, "bad pair");
    }
}
