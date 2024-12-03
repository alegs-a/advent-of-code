#![allow(warnings)]

use std::collections::HashSet;

pub fn part_1(input: String) -> String {
    solve_part_1(input, 64).to_string()
}

fn solve_part_1(input: String, depth: i32) -> usize {
    let mut walls: Vec<Coord> = Vec::new();
    let mut start = Coord::from(0, 0);
    let grid = Grid {
        height: input.lines().count() as i32,
        width: input.lines().next().unwrap().len() as i32,
    };
    for (y, line) in input.lines().enumerate() {
        for (x, character) in line.chars().enumerate() {
            if character == '#' {
                walls.push(Coord::from(y, x));
            }
            if character == 'S' {
                start = Coord::from(y, x);
            }
        }
    }
    let start = start;

    let mut positions: HashSet<Coord> = HashSet::new();
    positions.insert(start);

    for i in 0..depth {
        let temp_positions = positions.clone();
        positions.clear();
        //dbg!(i);

        for position in temp_positions {
            for neighbour in grid.neighbours(position) {
                if !walls.contains(&neighbour) {
                    //dbg!(&neighbour);
                    positions.insert(neighbour);
                }
            }
        }
    }
    positions.len()
}

pub fn part_2(input: String) -> String {
    String::new()
}

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Coord {
    y: usize,
    x: usize,
}
impl Coord {
    fn from(y: usize, x: usize) -> Coord {
        Coord { y, x }
    }
}
struct Grid {
    height: i32,
    width: i32,
}

impl Grid {
    fn neighbours(&self, coord: Coord) -> Vec<Coord> {
        let x = coord.x as i32;
        let y = coord.y as i32;

        let mut coords = Vec::new();

        if let Some(north_neighbour) = self.north_neighbour(y, x) {
            coords.push(north_neighbour);
        }
        if let Some(south_neighbour) = self.south_neighbour(y, x) {
            coords.push(south_neighbour);
        }
        if let Some(east_neighbour) = self.east_neighbour(y, x) {
            coords.push(east_neighbour);
        }
        if let Some(west_neighbour) = self.west_neighbour(y, x) {
            coords.push(west_neighbour);
        }

        coords
    }

    fn north_neighbour(&self, y: i32, x: i32) -> Option<Coord> {
        let y = y - 1;
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(Coord {
            x: x.try_into().ok()?,
            y: y.try_into().ok()?,
        })
    }
    fn south_neighbour(&self, y: i32, x: i32) -> Option<Coord> {
        let y = y + 1;
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(Coord {
            x: x.try_into().ok()?,
            y: y.try_into().ok()?,
        })
    }
    fn east_neighbour(&self, y: i32, x: i32) -> Option<Coord> {
        let x = x + 1;
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(Coord {
            x: x.try_into().ok()?,
            y: y.try_into().ok()?,
        })
    }
    fn west_neighbour(&self, y: i32, x: i32) -> Option<Coord> {
        let x = x - 1;
        if x < 0 || x >= self.width || y < 0 || y >= self.height {
            return None;
        }
        Some(Coord {
            x: x.try_into().ok()?,
            y: y.try_into().ok()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT.to_string(), 6), 16)
    }
}
