use std::{char, fs};

pub fn day_three() {
    println!("DAY 3 ===========");

    let raw_input = fs::read_to_string("input/3.txt").expect("error reading input file");

    part_one(raw_input.clone());
    part_two(raw_input);
}

fn part_one(input: String) {
    let mut score = 0;

    for line in input.lines() {
        let length = line.len();
        let half_lenth: usize = (length as f64 / 2.0).trunc() as usize;
        let halves = line.split_at(half_lenth);
        println!("{:#?}", halves);

        let duplicate = find_duplicate(halves);
        println!("{duplicate}");

        score += priority(duplicate);
    }
    println!("PART 1: The total priority is: {score}");
}

fn part_two(input: String) {
    let mut score = 0;

    let lines: Vec<&str> = input.lines().collect();
    let num_groups: usize = (lines.len() as f64 / 3.0).trunc() as usize;
    for i in 0..num_groups {
        score += priority(find_duplicate_extended((
            lines[i * 3],
            lines[i * 3 + 1],
            lines[i * 3 + 2],
        )));
    }
    println!("PART 2: The total priority is: {score}");
}

fn find_duplicate(slices: (&str, &str)) -> char {
    for character in slices.0.chars() {
        if slices.1.contains(character) {
            return character;
        }
    }
    panic!(
        "Something gone wrong: no match found in pattern {:#?}",
        slices
    );
}

fn find_duplicate_extended(strings: (&str, &str, &str)) -> char {
    let mut initial_matches: Vec<char> = Vec::new();
    for character in strings.0.chars() {
        if strings.1.contains(character) {
            initial_matches.push(character);
        }
    }
    for character in initial_matches {
        if strings.2.contains(character) {
            return character;
        }
    }
    panic!(
        "Uh oh! No matches found in part 2 for the set {:#?}",
        strings
    );
}

fn priority(input: char) -> i32 {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    alphabet.find(input).unwrap() as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::priority;

    #[test]
    fn basic_priority() {
        assert_eq!(priority('a'), 1);
        assert_eq!(priority('z'), 26);
        assert_eq!(priority('A'), 27);
    }
}
