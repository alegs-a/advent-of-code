use itertools::Itertools;

pub fn part_1(input: String) -> String {
    let mut grid = Grid::from(input);
    grid.fill_empty_rows_1();
    let coords = grid.coords();

    coords
        .iter()
        .combinations(2)
        .map(|x| {
            let a = x[0];
            let b = x[1];
            a.distance(b)
        })
        .sum::<i32>()
        .to_string()
}

pub fn part_2(input: String) -> String {
    let grid = Grid::from(input);
    let coords = grid.coords();

    let pairs: Vec<(Coord, Coord)> = coords
        .iter()
        .combinations(2)
        .map(|x| (*x[0], *x[1]))
        .collect();
    dbg!(&pairs.len());
    let mut lines_crossed = 0;
    for (a, b) in pairs {
        lines_crossed += grid.empty_cols_between(a, b) + grid.empty_rows_between(a, b)
    }
    dbg!(lines_crossed);

    let base_distance = coords
        .iter()
        .combinations(2)
        .map(|x| {
            let a = x[0];
            let b = x[1];
            a.distance(b)
        })
        .sum::<i32>();

    (base_distance as u128 + ((1000000-1) * lines_crossed as u128)).to_string()
}

//fn parse(input: String) -> Vec<Galaxy> {}

#[derive(PartialEq, Debug)]
struct Grid {
    /// True if galaxy, false if not
    data: Vec<Vec<bool>>,
}

impl Grid {
    fn from(data: String) -> Grid {
        let data = data
            .lines()
            .map(|line| {
                line.chars()
                    .map(|x| match x {
                        '#' => true,
                        '.' => false,
                        _ => panic!("invalid input soz"),
                    })
                    .collect::<Vec<bool>>()
            })
            .collect::<Vec<Vec<bool>>>();
        Grid { data }
    }

    #[allow(unused)]
    fn to_string(&self) -> String {
        let mut data = Vec::new();
        for y in 0..self.data.len() {
            let mut row = Vec::new();
            for x in 0..self.data[0].len() {
                if self.data[y][x] {
                    row.push('#');
                } else {
                    row.push('.');
                }
            }
            row.push('\n');
            data.push(row);
        }
        data.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<String>()
    }

    fn coords(&self) -> Vec<Coord> {
        let mut coords = Vec::new();
        for y in 0..self.data.len() {
            for x in 0..self.data[0].len() {
                if self.data[y][x] {
                    coords.push(Coord {
                        y: y as i32,
                        x: x as i32,
                    })
                }
            }
        }
        coords
    }

    fn empty_rows_between(&self, a: Coord, b: Coord) -> i32 {
        let empty_rows = self.get_empty_rows();
        empty_rows
            .iter()
            .filter(|x| {
                let x = **x as i32;
                (a.y < x && x < b.y) || (b.y < x && x < a.y)
            })
            .count() as i32
    }

    fn empty_cols_between(&self, a: Coord, b: Coord) -> i32 {
        let empty_cols = self.get_empty_cols();
        empty_cols
            .iter()
            .filter(|x| {
                let x = **x as i32;
                (a.x < x && x < b.x) || (b.x < x && x < a.x)
            })
            .count() as i32
    }

    fn get_empty_rows(&self) -> Vec<usize> {
        let mut rows_to_add = Vec::new();

        for y in 0..self.data.len() {
            if self.row_is_empty(y) {
                rows_to_add.push(y);
            }
        }
        rows_to_add
    }
    fn get_empty_cols(&self) -> Vec<usize> {
        let mut cols_to_add = Vec::new();

        for x in 0..self.data[0].len() {
            if self.column_is_empty(x) {
                cols_to_add.push(x);
            }
        }
        cols_to_add
    }

    fn fill_empty_rows_1(&mut self) {
        let rows_to_add = self.get_empty_rows();
        let cols_to_add = self.get_empty_cols();

        for (i, y) in rows_to_add.iter().enumerate() {
            self.insert_row_after(y + i);
        }
        for (i, x) in cols_to_add.iter().enumerate() {
            self.insert_column_after(x + i);
        }
    }

    fn insert_row_after(&mut self, row: usize) {
        self.data.insert(row + 1, vec![false; self.data[0].len()]);
    }

    fn insert_column_after(&mut self, col: usize) {
        for row in self.data.iter_mut() {
            row.insert(col + 1, false);
        }
    }

    fn row_is_empty(&self, row: usize) -> bool {
        let row = self.data.get(row).unwrap();
        !row.contains(&true)
    }

    fn column_is_empty(&self, col: usize) -> bool {
        let row: Vec<bool> = self.data.iter().map(|x| *x.get(col).unwrap()).collect();
        !row.contains(&true)
    }
}

#[derive(Clone, Copy, Debug)]
struct Coord {
    y: i32,
    x: i32,
}

impl Coord {
    fn distance(&self, other: &Coord) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_grid_expanding() {
        let expected = Grid::from(
            "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#......."
                .to_string(),
        );
        let mut grid = Grid::from(INPUT.to_string());
        grid.fill_empty_rows_1();
        println!("{}", grid.to_string());
        println!("{}", expected.to_string());
        assert_eq!(grid, expected);
    }

    #[test]
    fn test_get_empty_rows() {
        let grid = Grid::from(INPUT.to_string());
        let empty_rows = grid.get_empty_rows();
        let expected = vec![3, 7];

        assert_eq!(empty_rows, expected);
    }

    #[test]
    fn test_row_is_empty() {
        let grid = Grid::from(INPUT.to_string());
        assert!(grid.row_is_empty(3));
    }

    #[test]
    fn test_empty_rows_between() {
        let grid = Grid::from(INPUT.to_string());
        let a = Coord { x: 0, y: 0 };
        let b = Coord { x: 0, y: 4 };
        let c = Coord { x: 3, y: 9 };
        assert_eq!(grid.empty_rows_between(a, b), 1);
        assert_eq!(grid.empty_rows_between(b, a), 1);
        assert_eq!(grid.empty_rows_between(a, c), 2);
        assert_eq!(grid.empty_rows_between(c, a), 2);
    }

    #[test]
    fn test_empty_cols_between() {
        let grid = Grid::from(INPUT.to_string());
        let a = Coord { x: 0, y: 0 };
        let b = Coord { x: 3, y: 0 };
        let c = Coord { x: 9, y: 8 };
        assert_eq!(grid.empty_cols_between(a, b), 1);
        assert_eq!(grid.empty_cols_between(b, a), 1);
        assert_eq!(grid.empty_cols_between(a, c), 3);
        assert_eq!(grid.empty_cols_between(c, a), 3);
    }

    #[test]
    fn test_part_two_scale_10() {
        let grid = Grid::from(INPUT.to_string());
        let coords = grid.coords();

        let ans = coords
            .iter()
            .combinations(2)
            .map(|x| {
                let a = x[0];
                let b = x[1];

                (a.distance(b)
                    + 10 * grid.empty_rows_between(*a, *b)
                    + 10 * grid.empty_cols_between(*a, *b)) as u128
            })
            .sum::<u128>();
        assert_eq!(ans, 1030)
    }
    #[test]
    fn test_part_two_scale_100() {
        let grid = Grid::from(INPUT.to_string());
        let coords = grid.coords();

        let ans = coords
            .iter()
            .combinations(2)
            .map(|x| {
                let a = x[0];
                let b = x[1];

                (a.distance(b)
                    + 100 * grid.empty_rows_between(*a, *b)
                    + 100 * grid.empty_cols_between(*a, *b)) as u128
            })
            .sum::<u128>();
        assert_eq!(ans, 8410)
    }
}
