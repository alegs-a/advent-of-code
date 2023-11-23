#![allow(dead_code)]

use std::fs;

pub fn day_nine() {
    let raw_input = fs::read_to_string("input/9.txt").expect("Failed to read input file");

    println!("Part 1: {}", part_one(raw_input.clone()));
    println!("Part 2: {}", part_two(raw_input));
}
pub fn part_1(raw_input: String) -> String {
    let mut tail_covered: Vec<Coord> = vec![Coord::new_custom(0, 0)];

    let mut rope = Rope::new();

    for line in raw_input.lines() {
        let mut words = line.split_whitespace();

        let move_direction = parse_move(words.next().unwrap());
        let move_number = words.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..move_number {
            rope = move_rope(rope, move_direction);
            if !tail_covered.contains(&rope.tail) {
                tail_covered.push(rope.tail);
            }
            // visualise_visited(&tail_covered);
        }
    }
    tail_covered.len().to_string()
}

pub fn part_2(raw_input: String) -> String {
    let mut big_rope = vec![Coord::new(); 10];
    let mut tail_covered: Vec<Coord> = Vec::new();

    for line in raw_input.lines() {
        let mut words = line.split_whitespace();

        let move_direction = parse_move(words.next().unwrap());
        let move_number = words.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..move_number {
            big_rope[0] = match move_direction {
                Direction::Left => Coord::new_custom(big_rope[0].x - 1, big_rope[0].y),
                Direction::Right => Coord::new_custom(big_rope[0].x + 1, big_rope[0].y),
                Direction::Up => Coord::new_custom(big_rope[0].x, big_rope[0].y + 1),
                Direction::Down => Coord::new_custom(big_rope[0].x, big_rope[0].y - 1),
            };
            //dbg!(big_rope[0]);

            for i in 1..big_rope.len() {
                // x stuff
                let x_delta = big_rope[i - 1].x - big_rope[i].x;
                let y_delta = big_rope[i - 1].y - big_rope[i].y;
                // dbg!(move_direction, move_number, i, &big_rope, x_delta, y_delta);
                assert!(x_delta <= 2);
                assert!(x_delta >= -2);
                assert!(y_delta <= 2);
                assert!(y_delta >= -2);

                if x_delta == 1 && y_delta == 2
                    || x_delta == 2 && y_delta == 1
                    || x_delta == 2 && y_delta == 2
                {
                    big_rope[i].x += 1;
                    big_rope[i].y += 1;
                } else if x_delta == -1 && y_delta == 2
                    || x_delta == -2 && y_delta == 1
                    || x_delta == -2 && y_delta == 2
                {
                    big_rope[i].x -= 1;
                    big_rope[i].y += 1;
                } else if x_delta == 1 && y_delta == -2
                    || x_delta == 2 && y_delta == -1
                    || x_delta == 2 && y_delta == -2
                {
                    big_rope[i].x += 1;
                    big_rope[i].y -= 1;
                } else if x_delta == -1 && y_delta == -2
                    || x_delta == -2 && y_delta == -1
                    || x_delta == -2 && y_delta == -2
                {
                    big_rope[i].x -= 1;
                    big_rope[i].y -= 1;
                } else if x_delta == 2 {
                    big_rope[i].x += 1;
                } else if x_delta == -2 {
                    big_rope[i].x -= 1;
                } else if y_delta == 2 {
                    big_rope[i].y += 1;
                } else if y_delta == -2 {
                    big_rope[i].y -= 1;
                }
                let x_delta = big_rope[i - 1].x - big_rope[i].x;
                let y_delta = big_rope[i - 1].y - big_rope[i].y;
                assert!(-1 <= x_delta);
                assert!(x_delta <= 1);
                assert!(-1 <= y_delta);
                assert!(y_delta <= 1);
            }

            if !tail_covered.contains(big_rope.last().unwrap()) {
                tail_covered.push(*big_rope.last().unwrap());
            }
        }
    }
    // dbg!(big_rope);
    tail_covered.len().to_string()
}

fn part_one(raw_input: String) -> i32 {
    let mut tail_covered: Vec<Coord> = vec![Coord::new_custom(0, 0)];

    let mut rope = Rope::new();

    for line in raw_input.lines() {
        let mut words = line.split_whitespace();

        let move_direction = parse_move(words.next().unwrap());
        let move_number = words.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..move_number {
            rope = move_rope(rope, move_direction);
            if !tail_covered.contains(&rope.tail) {
                tail_covered.push(rope.tail);
            }
            // visualise_visited(&tail_covered);
        }
    }
    tail_covered.len() as i32
}

fn part_two(raw_input: String) -> i32 {
    let mut big_rope = vec![Coord::new(); 10];
    let mut tail_covered: Vec<Coord> = Vec::new();

    for line in raw_input.lines() {
        let mut words = line.split_whitespace();

        let move_direction = parse_move(words.next().unwrap());
        let move_number = words.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..move_number {
            big_rope[0] = match move_direction {
                Direction::Left => Coord::new_custom(big_rope[0].x - 1, big_rope[0].y),
                Direction::Right => Coord::new_custom(big_rope[0].x + 1, big_rope[0].y),
                Direction::Up => Coord::new_custom(big_rope[0].x, big_rope[0].y + 1),
                Direction::Down => Coord::new_custom(big_rope[0].x, big_rope[0].y - 1),
            };
            //dbg!(big_rope[0]);

            for i in 1..big_rope.len() {
                // x stuff
                let x_delta = big_rope[i - 1].x - big_rope[i].x;
                let y_delta = big_rope[i - 1].y - big_rope[i].y;
                // dbg!(move_direction, move_number, i, &big_rope, x_delta, y_delta);
                assert!(x_delta <= 2);
                assert!(x_delta >= -2);
                assert!(y_delta <= 2);
                assert!(y_delta >= -2);

                if x_delta == 1 && y_delta == 2
                    || x_delta == 2 && y_delta == 1
                    || x_delta == 2 && y_delta == 2
                {
                    big_rope[i].x += 1;
                    big_rope[i].y += 1;
                } else if x_delta == -1 && y_delta == 2
                    || x_delta == -2 && y_delta == 1
                    || x_delta == -2 && y_delta == 2
                {
                    big_rope[i].x -= 1;
                    big_rope[i].y += 1;
                } else if x_delta == 1 && y_delta == -2
                    || x_delta == 2 && y_delta == -1
                    || x_delta == 2 && y_delta == -2
                {
                    big_rope[i].x += 1;
                    big_rope[i].y -= 1;
                } else if x_delta == -1 && y_delta == -2
                    || x_delta == -2 && y_delta == -1
                    || x_delta == -2 && y_delta == -2
                {
                    big_rope[i].x -= 1;
                    big_rope[i].y -= 1;
                } else if x_delta == 2 {
                    big_rope[i].x += 1;
                } else if x_delta == -2 {
                    big_rope[i].x -= 1;
                } else if y_delta == 2 {
                    big_rope[i].y += 1;
                } else if y_delta == -2 {
                    big_rope[i].y -= 1;
                }
                let x_delta = big_rope[i - 1].x - big_rope[i].x;
                let y_delta = big_rope[i - 1].y - big_rope[i].y;
                assert!(-1 <= x_delta);
                assert!(x_delta <= 1);
                assert!(-1 <= y_delta);
                assert!(y_delta <= 1);
            }

            if !tail_covered.contains(big_rope.last().unwrap()) {
                tail_covered.push(*big_rope.last().unwrap());
            }
        }
    }
    // dbg!(big_rope);
    tail_covered.len() as i32
}

#[allow(dead_code)]
fn visualise_visited(input: &Vec<Coord>) {
    println!("Visualising input:");
    for y in 0..6 {
        for x in 0..7 {
            if input.contains(&Coord::new_custom(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
}

fn parse_move(input: &str) -> Direction {
    match input {
        "L" => Direction::Left,
        "R" => Direction::Right,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Invalid direction"),
    }
}

/// Move a rope one unit in the direction provided
fn move_rope(input: Rope, direction: Direction) -> Rope {
    let new_head: Coord;
    let new_tail: Coord;
    match direction {
        Direction::Left => {
            new_head = Coord::new_custom(input.head.x - 1, input.head.y);
            if (new_head.x - input.tail.x).abs() > 1 {
                // if tail needs to move
                new_tail = Coord::new_custom(new_head.x + 1, new_head.y);
            } else {
                new_tail = input.tail;
            }
        }
        Direction::Right => {
            new_head = Coord::new_custom(input.head.x + 1, input.head.y);
            if (new_head.x - input.tail.x).abs() > 1 {
                // if tail needs to move
                new_tail = Coord::new_custom(new_head.x - 1, new_head.y);
            } else {
                new_tail = input.tail;
            }
        }
        Direction::Up => {
            new_head = Coord::new_custom(input.head.x, input.head.y + 1);
            if (new_head.y - input.tail.y).abs() > 1 {
                // if tail needs to move
                new_tail = Coord::new_custom(new_head.x, new_head.y - 1);
            } else {
                new_tail = input.tail;
            }
        }
        Direction::Down => {
            new_head = Coord::new_custom(input.head.x, input.head.y - 1);
            if (new_head.y - input.tail.y).abs() > 1 {
                // if tail needs to move
                new_tail = Coord::new_custom(new_head.x, new_head.y + 1);
            } else {
                new_tail = input.tail;
            }
        }
    }
    Rope {
        head: new_head,
        tail: new_tail,
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    /// Returns a new Coord at (0, 0)
    fn new() -> Coord {
        Coord { x: 0, y: 0 }
    }

    /// Returns a new Coord at (x, y)
    fn new_custom(x: i32, y: i32) -> Coord {
        Coord { x, y }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Rope {
    head: Coord,
    tail: Coord,
}

impl Rope {
    /// Returns a new Rope with both the head and the tail at (0, 0)
    fn new() -> Rope {
        Rope {
            head: Coord::new(),
            tail: Coord::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::year_2022::day_9::Coord;

    use super::*;

    #[test]
    fn day_nine_part_one() {
        let input = String::from(
            "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
        );
        assert_eq!(part_one(input), 13);
    }

    #[test]
    fn day_nine_part_two() {
        let input = String::from(
            "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
        );
        assert_eq!(part_two(input), 36);
    }

    #[test]
    fn move_straight_right() {
        let input = Rope {
            head: Coord::new_custom(1, 0),
            tail: Coord::new_custom(0, 0),
        };
        let direction = Direction::Right;
        let expected = Rope {
            head: Coord::new_custom(2, 0),
            tail: Coord::new_custom(1, 0),
        };
        assert_eq!(move_rope(input, direction), expected);
    }

    #[test]
    fn move_straight_down() {
        let input = Rope {
            head: Coord::new_custom(0, 0),
            tail: Coord::new_custom(0, 1),
        };
        let direction = Direction::Down;
        let expected = Rope {
            head: Coord::new_custom(0, -1),
            tail: Coord::new_custom(0, 0),
        };
        assert_eq!(move_rope(input, direction), expected);
    }
}
