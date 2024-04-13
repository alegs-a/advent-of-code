use std::collections::HashMap;

pub fn part_1(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let line = line
            .trim_start_matches(char::is_alphabetic)
            .trim_end_matches(char::is_alphabetic);
        let num =
            line.chars().next().unwrap().to_string() + &line.chars().last().unwrap().to_string();
        sum += str::parse::<i32>(&num).unwrap();
    }
    sum.to_string()
}

pub fn part_2(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let mut nums = HashMap::new();
        nums.insert("one", "1");
        nums.insert("two", "2");
        nums.insert("three", "3");
        nums.insert("four", "4");
        nums.insert("five", "5");
        nums.insert("six", "6");
        nums.insert("seven", "7");
        nums.insert("eight", "8");
        nums.insert("nine", "9");
        let mut line = String::from(line);
        let first_str = first_num(line.clone());
        let last_str = last_num(line.clone());
        if let Some(first_str) = first_str {
            line = line.replace(&first_str, nums.get(first_str.as_str()).unwrap());
        }
        if let Some(last_str) = last_str {
            line = line.replace(&last_str, nums.get(last_str.as_str()).unwrap());
        }
        //         let replaced = line
        //             .replace("two", "2")
        //             .replace("one", "1")
        //             .replace("eight", "8")
        //             .replace("three", "3")
        //             .replace("four", "4")
        //             .replace("five", "5")
        //             .replace("six", "6")
        //             .replace("nine", "9")
        //             .replace("seven", "7");
        let line = line
            .trim_start_matches(char::is_alphabetic)
            .trim_end_matches(char::is_alphabetic);
        let num =
            line.chars().next().unwrap().to_string() + &line.chars().last().unwrap().to_string();
        //dbg!(&num);
        sum += str::parse::<i32>(&num).unwrap();
    }
    sum.to_string()
}

/// Returns the first number to be spelt out in the string, or None if there are no numbers
fn first_num(string: String) -> Option<String> {
    let mut nums = HashMap::new();
    nums.insert("one", "1");
    nums.insert("two", "2");
    nums.insert("three", "3");
    nums.insert("four", "4");
    nums.insert("five", "5");
    nums.insert("six", "6");
    nums.insert("seven", "7");
    nums.insert("eight", "8");
    nums.insert("nine", "9");

    let mut first_index = usize::MAX;
    let mut first_str = None;

    for foo in nums.keys() {
        if let Some(current_first_index) = string.find(foo) {
            if current_first_index < first_index {
                first_index = current_first_index;
                first_str = Some(foo.to_string());
            }
        }
    }
    first_str
}

fn last_num(string: String) -> Option<String> {
    let string = string.chars().rev().collect::<String>();
    let mut rev_nums = HashMap::new();
    rev_nums.insert("eno", "1");
    rev_nums.insert("owt", "2");
    rev_nums.insert("eerht", "3");
    rev_nums.insert("ruof", "4");
    rev_nums.insert("evif", "5");
    rev_nums.insert("xis", "6");
    rev_nums.insert("neves", "7");
    rev_nums.insert("thgie", "8");
    rev_nums.insert("enin", "9");

    let mut first_index = usize::MAX;
    let mut first_str = None;

    for foo in rev_nums.keys() {
        if let Some(current_first_index) = string.find(*foo) {
            if current_first_index < first_index {
                first_index = current_first_index;
                first_str = Some(foo.to_string());
            }
        }
    }
    if let Some(first_str) = first_str {
        return Some(first_str.chars().rev().collect::<String>());
    }
    first_str
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_two() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            .to_string();
        assert_eq!(part_2(input), "281");
    }
}
