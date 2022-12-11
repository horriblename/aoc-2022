use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

// I just wanna try with different sizes
type WorryLvl = u64;

#[derive(Debug)]
struct Monkey {
    _id: u32,
    items: VecDeque<WorryLvl>,
    operation: Operation,
    test_divisible_by: u32,
    if_true: u32,  // the monkey's id to throw to if test is true
    if_false: u32, // the monkey's id to throw to if test is false
    inspected: usize,
}

#[derive(Debug)]
enum Operation {
    Square,
    Add(WorryLvl),
    Multiply(WorryLvl),
}
use Operation::*;

fn parse_operation(line: &str) -> Operation {
    let operator = line.as_bytes()[23];
    let rhs = &line[25..];

    if rhs == "old" {
        // there's only `old * old` that exists in my input
        return Square;
    }

    let rhs = rhs.parse().unwrap();

    match operator {
        b'+' => Add(rhs),
        b'*' => Multiply(rhs),
        _ => panic!("unrecognized operator!"),
    }
}

fn parse_input(fname: &str) -> Vec<Monkey> {
    let file = File::open(fname).unwrap();
    let mut lines = BufReader::new(file).lines();

    let mut monkeys: Vec<Monkey> = Vec::new();

    let parse_unwrap = |res: &str| res.parse().expect("error parsing string into u32");

    loop {
        let mut get_line = || {
            lines
                .next()
                .expect("no more input lines")
                .expect("error reading input file")
        }; // FIXME

        let monke = parse_unwrap(&get_line()[7..8]); // there are less than 10 monkeys, just take 1
                                                     // digit
        let items = get_line()[18..]
            .split(',')
            .map(|item| item.trim().parse().expect("error parsing string into u64"))
            .collect();
        let operation = parse_operation(&get_line()[..]);
        let test_divisible_by = parse_unwrap(&get_line()[21..]);
        let if_true = parse_unwrap(&get_line()[29..]);
        let if_false = parse_unwrap(&get_line()[30..]);

        monkeys.push(Monkey {
            _id: monke,
            items,
            operation,
            test_divisible_by,
            if_true,
            if_false,
            inspected: 0,
        });

        if lines.next().is_none() {
            break;
        }
    }

    monkeys
}

fn simulate_monkey_rounds(mut monkeys: Vec<Monkey>, rounds: usize, relief: bool) -> Vec<Monkey> {
    // the "worry level" will eventually get too big for even u128 to handle, we need to
    // reduce the value so that that doesn't happen
    let lcm = monkeys
        .iter()
        .map(|monke| monke.test_divisible_by as WorryLvl)
        .fold(1, |acc_lcm, x| lcm(acc_lcm, x));

    for round in 0..rounds * monkeys.len() {
        let turn = round % monkeys.len();
        // let mut thrower = monkeys.get_mut(turn).unwrap();

        while let Some(mut to_throw) = monkeys.get_mut(turn).unwrap().items.pop_front() {
            let mut thrower = monkeys.get_mut(turn).unwrap();
            // inspect (increase worry according to operation)
            match &thrower.operation {
                Square => to_throw *= to_throw,
                Add(x) => to_throw += *x,
                Multiply(x) => to_throw *= *x,
            };
            to_throw %= lcm as WorryLvl; // FIXME
            thrower.inspected += 1;

            // relief (worry level /= 3)
            if relief {
                to_throw /= 3;
            }

            // test worry level
            let rcv_idx = if to_throw % thrower.test_divisible_by as WorryLvl == 0 {
                thrower.if_true as usize
            } else {
                thrower.if_false as usize
            };

            let receiver = monkeys.get_mut(rcv_idx).unwrap();
            receiver.items.push_back(to_throw);
        }
    }
    monkeys
}

fn solve(fname: &str, task_num: u8) -> usize {
    //
    let monkeys = parse_input(fname);

    let monkeys = if task_num == 1 {
        // task 1
        simulate_monkey_rounds(monkeys, 20, true)
    } else {
        // task 2
        simulate_monkey_rounds(monkeys, 10000, false)
    };

    let max_two = monkeys.iter().fold([0, 0], |max_two, monkey| {
        if monkey.inspected <= max_two[0] {
            max_two
        } else {
            [
                max_two[1].min(monkey.inspected),
                max_two[1].max(monkey.inspected),
            ]
        }
    });

    max_two[0] * max_two[1]
}

fn main() {
    let x = solve("input.txt", 1);
    println!("{}", x);
    let x = solve("input.txt", 2);
    println!("{}", x);
}

/// ripped straight from https://www.hackertouch.com/lowest-common-multiple-in-rust.html
fn lcm(first: WorryLvl, second: WorryLvl) -> WorryLvl {
    first * second / gcd(first, second)
}

fn gcd(first: WorryLvl, second: WorryLvl) -> WorryLvl {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, simulate_monkey_rounds, solve};

    #[test]
    fn test_parse_input() {
        let monkeys = parse_input("test_input.txt");
        // check the output yourself :P
        println!("{:#?}", monkeys);
    }

    #[test]
    fn test_simulate_monkey_rounds() {
        let monkeys = parse_input("test_input.txt");
        let inspect_counts: Vec<usize> = simulate_monkey_rounds(monkeys, 20, true)
            .iter()
            .map(|monke| monke.inspected)
            .collect();
        assert_eq!(inspect_counts, vec![101, 95, 7, 105]);
    }

    #[test]
    fn test_solve1() {
        assert_eq!(solve("test_input.txt", 1), 10605)
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve("test_input.txt", 2), 2713310158)
    }
}
