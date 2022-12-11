use std::{
    fs::File,
    io::{BufRead, BufReader},
};

enum Instruction {
    Add(i32),
    Noop,
}
use Instruction::{Add, Noop};

fn parse_line(line: &str) -> Instruction {
    let tokens: Vec<&str> = line.split(' ').collect();
    match tokens[0] {
        "addx" => Add(tokens[1].parse().expect("Error parsing token into i32")),
        "noop" => Noop,
        _ => panic!("Unknown instruction in input file"),
    }
}

fn is_interesting(cycle: i32) -> bool {
    cycle % 40 == 20
}

///@return Option<(i32, i32)> a tuple (cycle, reg) which is the "interesting"
/// cycle we run into during execution of the instruction, and their register value
/// None if we didn't run into an "interesting" cycle.
fn execute_instruction(cycle: &mut i32, reg: &mut i32, command: Instruction) -> Option<(i32, i32)> {
    let mut interesting = None;
    match command {
        Add(x) => {
            if is_interesting(*cycle + 1) {
                interesting = Some((*cycle + 1, *reg));
            }
            *reg += x;
            *cycle += 2;
        }
        Noop => *cycle += 1,
    }
    if is_interesting(*cycle) {
        interesting = Some((*cycle, *reg));
    }
    interesting
}

fn solve1(fname: &str) -> i32 {
    let file = File::open(fname).unwrap();
    let lines = BufReader::new(file).lines();
    let mut cycle = 1;
    let mut reg = 1;
    let mut sum = 0;

    for line in lines {
        let line = line.expect("error reading input file");
        let command = parse_line(&line);

        if let Some(x) = execute_instruction(&mut cycle, &mut reg, command) {
            sum += x.0 * x.1;
        };
    }

    sum
}

const CRT_COLUMNS: usize = 40;
const CRT_ROWS: usize = 6;
fn solve2(fname: &str) -> String {
    let file = File::open(fname).unwrap();
    let mut lines = BufReader::new(file).lines();
    let mut reg = 1;
    let mut pending_add: Option<i32> = None;
    let mut crt = vec![b' '; CRT_COLUMNS * CRT_ROWS];

    for cycle in 1..CRT_ROWS * CRT_COLUMNS + 1 {
        if let Some(x) = pending_add.take() {
            reg += x;
        } else {
            let line = lines
                .next()
                .expect("no more input in file")
                .expect("error reading input file");

            match parse_line(&line) {
                Noop => (),
                Add(x) => pending_add = Some(x),
            }
        }

        if ((cycle % CRT_COLUMNS) as i32).abs_diff(reg) <= 1 {
            // draw
            crt[cycle] = b'#';
        }
    }

    let lines: Vec<&str> = crt
        .chunks_exact(CRT_COLUMNS)
        .map(|line| std::str::from_utf8(line).unwrap())
        .collect();

    lines.join("\n")
}

fn main() {
    let x = solve1("input.txt");
    println!("{}", x);

    let y = solve2("input.txt");
    println!("{}", y);
}

#[cfg(test)]
mod test {
    use crate::solve1;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1("test_input.txt"), 13140);
    }
}
