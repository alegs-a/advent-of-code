#![allow(unused)]
use std::fs;

pub fn day_six() {
    let raw_input = fs::read_to_string("input/6.txt").expect("error reading input file");

    part_one(raw_input.clone());
    part_two(raw_input.clone());
}
pub fn part_1(raw_input: String) -> String {
    let mut found = false;
    let mut count = 0;
    let mut chars = Vec::new();
    let input = raw_input.chars().collect::<Vec<char>>();

    while !found {
        chars.push(input[count]);
        count += 1;

        if count > 4 {
            chars.remove(0);
            // dbg!(&chars);
            let mut sorted = chars.clone();
            sorted.sort();
            if !(sorted[0] == sorted[1] || sorted[1] == sorted[2] || sorted[2] == sorted[3]) {
                found = true;
                return count.to_string();
            }
        }
    }
    unreachable!("No answer found in part 1 of day 6")
}

pub fn part_2(raw_input: String) -> String {
    let mut found = false;
    let mut count = 0;
    let mut chars = Vec::new();
    let input = raw_input.chars().collect::<Vec<char>>();

    while !found {
        chars.push(input[count]);
        count += 1;

        if count > 14 {
            chars.remove(0);
            // dbg!(&chars);
            let mut sorted = chars.clone();
            sorted.sort();
            let mut deduped = chars.clone();
            deduped.sort();
            deduped.dedup();
            // dbg!(&sorted, &deduped);
            // dbg!(&sorted.len(), &deduped.len());

            if sorted.len() == deduped.len() {
                //println!("PART 2: {count}");
                found = true;
                return count.to_string();
            }
        }
    }
    panic!("No answer found in part 2");
}

fn part_one(raw_input: String) {
    let mut found = false;
    let mut count = 0;
    let mut chars = Vec::new();
    let input = raw_input.chars().collect::<Vec<char>>();

    while !found {
        chars.push(input[count]);
        count += 1;

        if count > 4 {
            chars.remove(0);
            // dbg!(&chars);
            let mut sorted = chars.clone();
            sorted.sort();
            if !(sorted[0] == sorted[1] || sorted[1] == sorted[2] || sorted[2] == sorted[3]) {
                found = true;
                println!("count: {count}");
            }
        }
    }
}

fn part_two(raw_input: String) -> i32 {
    let mut found = false;
    let mut count = 0;
    let mut chars = Vec::new();
    let input = raw_input.chars().collect::<Vec<char>>();

    while !found {
        chars.push(input[count]);
        count += 1;

        if count > 14 {
            chars.remove(0);
            // dbg!(&chars);
            let mut sorted = chars.clone();
            sorted.sort();
            let mut deduped = chars.clone();
            deduped.sort();
            deduped.dedup();
            // dbg!(&sorted, &deduped);
            // dbg!(&sorted.len(), &deduped.len());

            if sorted.len() == deduped.len() {
                println!("PART 2: {count}");
                found = true;
                return count as i32;
            }
        }
    }
    panic!("No answer found in part 2");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_one() {
        let input: String = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        assert_eq!(part_two(input), 19);
    }
    #[test]
    fn test_case_two() {
        let input: String = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        assert_eq!(part_two(input), 23);
    }
}
