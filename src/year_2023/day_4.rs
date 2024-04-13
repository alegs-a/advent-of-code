#![allow(warnings)]

use std::collections::HashSet;

pub fn part_1(input: String) -> String {
    let mut sum = 0;
    for line in input.lines() {
        let card = Card::new(line);
        sum += card.points();
    }
    sum.to_string()
}

pub fn part_2(input: String) -> String {
    let mut deck = Vec::new();
    for line in input.lines() {
        deck.push(Card::new(line));
    }

    let mut num_copies = vec![1; deck.len()];

    for (i, card) in deck.iter().enumerate() {
        for _ in 0..num_copies[i] {
            for duplicating in i + 1..1+card.num_matches() as usize {
                //dbg!(&num_copies);
                num_copies[duplicating] += 1;
            }
        }
        //dbg!(i, &num_copies);
    }

    num_copies.iter().sum::<i32>().to_string()
}

struct Card {
    nums: HashSet<i32>,
    winning_nums: HashSet<i32>,
}

impl Card {
    fn new(line: &str) -> Card {
        let Some((_, all_nums)) = line.split_once(": ") else {
            panic!()
        };
        let Some((winning_nums, card_nums)) = all_nums.split_once(" | ") else {
            panic!()
        };

        let nums: HashSet<i32> = card_nums
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let winning_nums: HashSet<i32> = winning_nums
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        Card {
            nums,
            winning_nums,
        }
    }

    fn points(&self) -> i32 {
        let count = self.num_matches();
        if count == 0 {
            0
        } else {
            2_i32.pow(count - 1)
        }
    }

    fn num_matches(&self) -> u32 {
        let mut count = 0;
        for num in &self.nums {
            if self.winning_nums.contains(&num) {
                count += 1;
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    //#[test]
    fn test_part_2() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part_2(input.to_string()), "30");
    }

}
