use std::{
    fs::File,
    io::{self, BufRead},
};

const ORD_ZERO: u8 = 48;

type Grid = Vec<Vec<u8>>;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::*;

// returns (x, y)
fn get_tree_pos_across_axis(from: &Direction, grid: &Grid) -> (Vec<usize>, Vec<usize>) {
    let h = grid.len();
    let w = grid[0].len();

    match from {
        Left | Up => ((0..w).collect(), (0..h).collect()),
        Right => ((0..w).rev().collect(), (0..h).collect()),
        Down => ((0..w).collect(), (0..h).rev().collect()),
    }
}

fn look_horizontal(grid: &Grid, map: &mut Vec<Vec<bool>>, xs: &[usize], ys: &[usize]) {
    for y in ys.iter().skip(1) {
        let mut curr_highest = grid[*y][xs[0]];
        for x in xs.iter().skip(1) {
            let height = grid[*y][*x];
            if height > curr_highest {
                curr_highest = height;
                map[*y][*x] = true;
            }
        }
    }
}

fn look_vertical(grid: &Grid, map: &mut Vec<Vec<bool>>, xs: &[usize], ys: &[usize]) {
    for x in xs.iter().skip(1) {
        let mut curr_highest = grid[ys[0]][*x];
        for y in ys.iter().skip(1) {
            let height = grid[*y][*x];
            if height > curr_highest {
                curr_highest = height;
                map[*y][*x] = true;
            }
        }
    }
}

fn look_and_mark(look_from: Direction, grid: &Grid, map: &mut Vec<Vec<bool>>) {
    let (xs, ys) = get_tree_pos_across_axis(&look_from, grid);

    match &look_from {
        Left | Right => look_horizontal(grid, map, &xs, &ys),
        Down | Up => look_vertical(grid, map, &xs, &ys),
    }
}

fn get_visible_trees(grid: &Grid) -> usize {
    let mut map = vec![vec![false; grid[0].len()]; grid.len()];
    map[0].fill(true);
    let last = map.len() - 1;
    map[last].fill(true);

    for row in map.iter_mut() {
        row[0] = true;
        let last = row.len() - 1;
        row[last] = true;
    }

    for dir in [Up, Down, Left, Right] {
        look_and_mark(dir, grid, &mut map);
    }

    map.iter().flatten().filter(|visible| **visible).count()
}

fn calculate_scenic_score(grid: &Grid, x: usize, y: usize) -> u32 {
    [Up, Down, Left, Right]
        .iter()
        .map(|dir| calculate_score_in_dir(grid, x, y, &dir))
        .product::<u32>()
}

fn calculate_score_in_dir(grid: &Grid, x: usize, y: usize, dir: &Direction) -> u32 {
    let treehouse_height = grid[y][x];
    let iter: Vec<&u8> = match &dir {
        Up => grid.iter().take(y).map(|row| &row[x]).rev().collect(),
        Down => grid.iter().skip(y + 1).map(|row| &row[x]).collect(),
        Left => grid[y].iter().take(x).rev().collect(),
        Right => grid[y].iter().skip(x + 1).collect(),
    };

    for (idx, tree) in iter.iter().enumerate() {
        if **tree >= treehouse_height {
            return idx as u32 + 1;
        }
    }
    iter.len() as u32
}

fn get_highest_scenic_score(grid: &Grid) -> u32 {
    let mut hiscore = 0;
    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let score = calculate_scenic_score(grid, x, y);
            if score > hiscore {
                hiscore = score;
            }
        }
    }

    hiscore
}

fn parse_input(fname: &str) -> Grid {
    let file = File::open(fname).unwrap();
    let lines = io::BufReader::new(file).lines();

    lines
        .filter_map(|line| match line {
            Ok(line) => Some(
                line.as_bytes()
                    .iter()
                    .map(|chr| chr - ORD_ZERO)
                    .collect::<Vec<u8>>(),
            ),
            Err(_) => None, // Not sure why it would fail
        })
        .collect()
}

fn main() {
    let grid = parse_input("input.txt");
    let ans = get_visible_trees(&grid);
    println!("{}", ans);

    let ans = get_highest_scenic_score(&grid);
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use crate::{get_highest_scenic_score, get_visible_trees};

    #[test]
    fn test_get_visible_trees() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(get_visible_trees(&grid), 21);
    }

    #[test]
    fn test_get_highest_scenic_score() {
        let grid = vec![
            vec![3, 0, 3, 7, 3],
            vec![2, 5, 5, 1, 2],
            vec![6, 5, 3, 3, 2],
            vec![3, 3, 5, 4, 9],
            vec![3, 5, 3, 9, 0],
        ];

        assert_eq!(get_highest_scenic_score(&grid), 8);
    }
}
