use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

type Coord = (i32, i32);

#[derive(PartialEq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

use Direction::{Down, Left, Right, Up};

// fn do_motion(head: &mut Coord, tail: &mut Coord, line: &str) ->

fn parse_line_and_execute(head: Coord, tail: Coord, line: &str) -> (Coord, Coord, HashSet<Coord>) {
    let (dir, n) = parse_motion(line);
    let mut trail = HashSet::new();
    trail.insert(tail);

    let mut head = head;
    let mut tail = tail;

    for _ in 0..n {
        (head, tail) = move_once(head, tail, &dir);
        trail.insert(tail);
    }

    (head, tail, trail)
}

fn move_once(head: Coord, tail: Coord, dir: &Direction) -> (Coord, Coord) {
    let new_head = match &dir {
        Up => (head.0, head.1 + 1),
        Right => (head.0 + 1, head.1),
        Down => (head.0, head.1 - 1),
        Left => (head.0 - 1, head.1),
    };
    let new_tail = tail_follow_new_head(new_head, tail);
    (new_head, new_tail)
}

fn are_adjacent(head: Coord, tail: Coord) -> bool {
    head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1
}

/// gives the new position of the tail given the new position of the head
fn tail_follow_new_head(new_head: Coord, tail: Coord) -> Coord {
    if are_adjacent(new_head, tail) {
        return tail;
    }

    let x_axis = new_head.0 - tail.0;
    let y_axis = new_head.1 - tail.1;

    let x_step = constrain_to_one(x_axis);
    let y_step = constrain_to_one(y_axis);

    (tail.0 + x_step, tail.1 + y_step)
}

fn constrain_to_one(n: i32) -> i32 {
    match n {
        0 => 0,
        1.. => 1,
        _ => -1,
    }
}

fn parse_motion(line: &str) -> (Direction, u32) {
    let dir = match &line[0..1] {
        "U" => Up,
        "R" => Right,
        "D" => Down,
        "L" => Left,
        _ => panic!("Error parsing direction"),
    };
    let n = line[2..].parse().expect("Error parsing into u32");
    (dir, n)
}

fn solve1(fname: &str) -> u32 {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();

    let mut trail: HashSet<Coord> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for line in lines {
        let line = line.expect("fatal error while reading file");
        let new_trails;
        (head, tail, new_trails) = parse_line_and_execute(head, tail, &line);
        trail.extend(new_trails);
    }

    trail.iter().count() as u32
}

fn solve2(fname: &str, rope_len: usize) -> u32 {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();

    let mut trail: HashSet<Coord> = HashSet::new();
    let mut rope = vec![(0, 0); rope_len];

    for line in lines {
        let line = line.expect("fatal error while reading file");

        let (dir, n) = parse_motion(&line);

        for _ in 0..n {
            move_rope_once(&mut rope, &dir);
            trail.insert(*rope.last().unwrap());
        }
    }

    trail.into_iter().count() as u32
}

fn move_rope_once(body: &mut Vec<Coord>, dir: &Direction) {
    (body[0], body[1]) = move_once(body[0], body[1], dir);
    let mut head = body[1];

    for knot in body.iter_mut().skip(2) {
        *knot = tail_follow_new_head(head, *knot);
        head = *knot;
    }
}

fn main() {
    let x = solve1("input.txt");
    println!("{}", x);
    let x = solve2("input.txt", 10);
    println!("{}", x);
}

#[cfg(test)]
mod tests {
    use crate::{are_adjacent, move_once, parse_line_and_execute, parse_motion, solve1, solve2};

    #[test]
    fn test_parse_line_and_execute() {
        let tests = [
            ((0, 0), (0, 0), "R 1", (1, 0), (0, 0)),
            ((4, 0), (3, 0), "U 4", (4, 4), (4, 3)),
            ((4, 4), (4, 3), "L 3", (1, 4), (2, 4)),
            ((1, 4), (2, 4), "D 1", (1, 3), (2, 4)),
            ((1, 3), (2, 4), "R 4", (5, 3), (4, 3)),
            ((5, 3), (4, 3), "D 1", (5, 2), (4, 3)),
            ((5, 2), (4, 3), "L 5", (0, 2), (1, 2)),
            ((0, 2), (1, 2), "R 2", (2, 2), (1, 2)),
        ];

        for test in tests {
            let head = test.0;
            let tail = test.1;
            let line = test.2;
            let (head, tail, _) = parse_line_and_execute(head, tail, line);
            assert_eq!(head, test.3);
            assert_eq!(tail, test.4);
        }
    }

    #[test]
    fn test_move_once() {
        use crate::Direction::*;
        let tests = [
            // general
            ((0, 0), (0, 0), Right, (1, 0), (0, 0)),
            ((0, 0), (0, 0), Up, (0, 1), (0, 0)),
            ((0, 0), (0, 0), Left, (-1, 0), (0, 0)),
            ((0, 0), (0, 0), Down, (0, -1), (0, 0)),
            // moving too far
            ((1, 1), (0, 0), Right, (2, 1), (1, 1)),
            ((1, 1), (0, 0), Up, (1, 2), (1, 1)),
            ((-1, -1), (0, 0), Left, (-2, -1), (-1, -1)),
            ((-1, -1), (0, 0), Down, (-1, -2), (-1, -1)),
            // moving within range
            ((1, 1), (0, 0), Left, (0, 1), (0, 0)),
            ((0, 0), (1, 1), Right, (1, 0), (1, 1)),
            ((0, 0), (1, 1), Up, (0, 1), (1, 1)),
            ((1, 1), (0, 0), Down, (1, 0), (0, 0)),
        ];

        for test in tests {
            let head = test.0;
            let tail = test.1;
            let dir = test.2;
            assert_eq!(move_once(head, tail, &dir), (test.3, test.4));
        }
    }

    #[test]
    fn test_are_adjacent() {
        let tests = [
            ((0, 1), (0, 0), true),
            ((-5, 9), (-6, 10), true),
            ((2, 0), (0, 0), false),
            ((5, 8), (7, 8), false),
            ((-1, -1), (1, 1), false),
        ];

        for test in tests {
            assert_eq!(are_adjacent(test.0, test.1), test.2);
        }
    }

    #[test]
    fn test_parse_motion() {
        use crate::Direction::*;
        let tests = [
            ("R 1", Right, 1),
            ("L 3", Left, 3),
            ("U 4", Up, 4),
            ("D 2", Down, 2),
        ];

        for test in tests {
            assert_eq!(parse_motion(test.0), (test.1, test.2));
        }
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("test_input.txt"), 13);
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2("test_input.txt", 10), 1);
        assert_eq!(solve2("test_input2.txt", 10), 36);
    }
}
