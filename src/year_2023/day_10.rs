//#![allow(warnings)]
use itertools::Itertools;

pub fn part_1(input: String) -> String {
    let grid = Grid::new(input);
    let mut coords = grid.find_loops();
    coords.push(grid.start);
    (coords.len() / 2).to_string()
}

pub fn part_2(input: String) -> String {
    let grid = Grid::new(input);
    let coords = grid.find_loops();

    let area = area(&coords);

    (area - (coords.len() as i32 / 2) + 1).to_string()
}

/// The area enclosed by the coordinates. Assumes they make a shape
fn area(coords: &Vec<Coord>) -> i32 {
    let mut coords = coords.clone();
    coords.push(coords[0]);
    let double_area: i32 = coords
        .iter()
        .tuple_windows()
        .map(|(first, second)| (first.y as i32 + second.y as i32) * (first.x as i32 - second.x as i32))
        .sum::<i32>()
        .abs();
    double_area / 2
}

enum Pipe {
    Vertical,
    Horizontal,
    CornerNE,
    CornerNW,
    CornerSE,
    CornerSW,
    Ground,
    Start,
}

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn deltas(&self) -> (i32, i32) {
        match *self {
            Self::North => (-1, 0),
            Self::South => (1, 0),
            Self::East => (0, 1),
            Self::West => (0, -1),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct Coord {
    y: usize,
    x: usize,
}

impl Coord {
    fn neighbour(&self, direction: Direction) -> Option<Coord> {
        let (y_delta, x_delta) = direction.deltas();
        let y = (self.y as i32 + y_delta).try_into().ok()?;
        let x = (self.x as i32 + x_delta).try_into().ok()?;
        Some(Coord { y, x })
    }
}

impl Pipe {
    /// This is ugly and awful and I am so sorry
    fn neighbours(&self, coord: Coord) -> (Option<Coord>, Option<Coord>) {
        match self {
            Pipe::Vertical => (
                coord.neighbour(Direction::North),
                coord.neighbour(Direction::South),
            ),
            Pipe::Horizontal => (
                coord.neighbour(Direction::East),
                coord.neighbour(Direction::West),
            ),
            Pipe::CornerNE => (
                coord.neighbour(Direction::North),
                coord.neighbour(Direction::East),
            ),
            Pipe::CornerNW => (
                coord.neighbour(Direction::North),
                coord.neighbour(Direction::West),
            ),
            Pipe::CornerSE => (
                coord.neighbour(Direction::South),
                coord.neighbour(Direction::East),
            ),
            Pipe::CornerSW => (
                coord.neighbour(Direction::South),
                coord.neighbour(Direction::West),
            ),
            Pipe::Ground => (None, None),
            Pipe::Start => unreachable!(),
        }
    }

    fn is_corner(&self) -> bool {
        match self {
            Self::CornerNE | Self::CornerNW | Self::CornerSE | Self::CornerSW => true,
            _ => false,
        }
    }
}

struct Grid {
    data: Vec<Vec<Pipe>>,
    start: Coord,
}

impl Grid {
    fn new(input: String) -> Grid {
        {
            let mut start_x: Option<usize> = None;
            let mut start_y: Option<usize> = None;
            let data = input
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(|(x, character)| match character {
                            '|' => Pipe::Vertical,
                            '-' => Pipe::Horizontal,
                            'L' => Pipe::CornerNE,
                            'J' => Pipe::CornerNW,
                            '7' => Pipe::CornerSW,
                            'F' => Pipe::CornerSE,
                            '.' => Pipe::Ground,
                            'S' => {
                                start_y = Some(y);
                                start_x = Some(x);
                                Pipe::Start
                            }
                            _ => unreachable!(),
                        })
                        .collect::<Vec<Pipe>>()
                })
                .collect::<Vec<Vec<Pipe>>>();
            let start = Coord {
                x: start_x.unwrap(),
                y: start_y.unwrap(),
            };
            Some(Grid { data, start })
        }
        .unwrap()
    }

    fn get(&self, coord: Coord) -> Option<&Pipe> {
        let row = self.data.get(coord.y)?;
        row.get(coord.x)
    }

    fn find_loops(&self) -> Vec<Coord> {
        let start_positions = vec![
            self.start.neighbour(Direction::North).unwrap(),
            self.start.neighbour(Direction::South).unwrap(),
            self.start.neighbour(Direction::East).unwrap(),
            //self.start.neighbour(Direction::West).unwrap(),
        ];
        for start in start_positions {
            if let Some(loop_sequence) = self.find_loop(start) {
                return loop_sequence;
            }
        }
        unreachable!("you done goofed");
    }

    fn find_loop(&self, start: Coord) -> Option<Vec<Coord>> {
        let mut current = start;
        let mut prev = self.start;
        let mut visited = vec![prev];
        let mut next = Coord {
            x: usize::MAX,
            y: usize::MAX,
        }; // forgive me my sins
        loop {
            //dbg!(current);
            let pipe = self.get(current).unwrap();
            let (neighbour_1, neighbour_2) = pipe.neighbours(current);
            // check both neighbours are not none
            if neighbour_1.is_none() || neighbour_2.is_none() {
                break;
            }
            let neighbour_1 = neighbour_1.unwrap();
            let neighbour_2 = neighbour_2.unwrap();

            // figure out which of neighbours is next and which is previous
            if !visited.contains(&neighbour_1) {
                next = neighbour_1;
            } else if !visited.contains(&neighbour_2) {
                next = neighbour_2;
            } else if neighbour_1 == self.start || neighbour_2 == self.start {
                visited.push(current);
                return Some(visited);
            } else {
                panic!("Something has gone very wrong :()")
            }

            // Check whether we've got back to the start
            if next == self.start {
                return Some(visited);
            }

            // check next and current have a valid connection
            let next_pipe = self.get(next).unwrap();
            let (next_pipe_neighbour_1, next_pipe_neighbour_2) = next_pipe.neighbours(next);
            let next_pipe_neighbours = [
                next_pipe_neighbour_1.unwrap(),
                next_pipe_neighbour_2.unwrap(),
            ];
            if !next_pipe_neighbours.contains(&current) {
                // No connection :(
                break;
            }

            // Debugging :(
            if current == (Coord { x: 55, y: 40 }) {
                print!("");
            }
            // continue (add current to visited, move next to current, current to prev)
            visited.push(current);
            prev = current;
            current = next;
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn find_loop() {
        let input = String::from(
            ".....
.S-7.
.|.|.
.L-J.
.....",
        );
        let grid = Grid::new(input);
        let start = Coord { x: 2, y: 1 };
        assert!(grid.find_loop(start).is_some());
    }

    #[test]
    fn test_area_small_square() {
        let input = "....
.S7.
.LJ.
....";
        let grid = Grid::new(input.to_string());
        let coords = grid.find_loops();
        dbg!(&coords);
        assert_eq!(area(&coords), 1);
    }
    //
    //    #[test]
    //    fn test_area_square() {
    //        let input = ".....
    //.S-7.
    //.|.|.
    //.L-J.
    //.....";
    //        let grid = Grid::new(input.to_string());
    //        let mut coords = grid.find_loops();
    //        coords.push(coords.first().unwrap().clone());
    //        assert_eq!(area(coords), 1);
    //    }
    //
    //    #[test]
    //    fn test_area_slightly_funky() {
    //        let input = "..F7.
    //.FJ|.
    //SJ.L7
    //|F--J
    //LJ...";
    //        let grid = Grid::new(input.to_string());
    //        let mut coords = grid.find_loops();
    //        coords.push(coords.first().unwrap().clone());
    //        assert_eq!(area(coords), 1);
    //    }
    //
    //    #[test]
    //    fn test_area_funky() {
    //        let input = "...........
    //.S-------7.
    //.|F-----7|.
    //.||.....||.
    //.||.....||.
    //.|L-7.F-J|.
    //.|..|.|..|.
    //.L--J.L--J.
    //...........";
    //        let grid = Grid::new(input.to_string());
    //        let mut coords = grid.find_loops();
    //        coords.push(coords.first().unwrap().clone());
    //        assert_eq!(area(coords), 4);
    //    }
}
