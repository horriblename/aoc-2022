use std::{
    fs::File,
    io::{self, BufRead},
};

fn crate_char_position(stack_index: usize) -> usize {
    1 + stack_index * 4
}

fn parse_stack(stacks_raw: &Vec<String>, i: i32) -> Vec<char> {
    let pos = crate_char_position(i as usize);
    let mut stack = vec![];

    for line in stacks_raw {
        let c = line.as_bytes()[pos];
        if c == ' ' as u8 {
            break;
        }
        stack.push(c as char);
    }
    stack
}

/// parses a line in the form of
/// "move x from y to z"
fn parse_movement(line: &str) -> Vec<usize> {
    line.split(' ')
        .filter_map(|x| x.parse::<usize>().ok())
        .collect()
}

/// perform a movement command, `from` and `to` are __1-indexed__
#[allow(dead_code)]
fn do_move(num: usize, stacks: &mut Vec<Vec<char>>, from: usize, to: usize) {
    if num == 0 {
        return;
    }
    let tmp = stacks[from - 1].pop().unwrap();
    stacks[to - 1].push(tmp);
    do_move(num - 1, stacks, from, to)
}

#[allow(dead_code)]
fn do_move2(num: usize, stacks: &mut Vec<Vec<char>>, from: usize, to: usize) {
    let at = stacks[from - 1].len() - num;
    let tmp = stacks[from - 1].split_off(at);
    stacks[to - 1].extend(tmp);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut lines = io::BufReader::new(file).lines();

    let mut stacks_raw = vec![];

    for _ in 0..8 {
        if let Some(Ok(line)) = lines.next() {
            stacks_raw.push(line);
        }
    }

    let lines = lines.skip(2);
    // parse and save to stack
    stacks_raw.reverse();
    let mut stacks = vec![];

    for i in 0..9 {
        stacks.push(parse_stack(&stacks_raw, i))
    }

    // read commands and execute them
    for line in lines {
        if let Ok(line) = line {
            let movement = parse_movement(&line[..]);
            let num = movement[0];
            let from = movement[1];
            let to = movement[2];
            do_move2(num, &mut stacks, from, to);
        }
    }

    let mut ans = String::from("");
    _ = stacks
        .into_iter()
        .map(|stack| ans.push(*stack.last().unwrap_or(&' ')))
        .collect::<()>();

    println!("{ans}");
}

mod tests {
    #[test]
    fn test_parse_movement() {
        use crate::parse_movement;

        assert_eq!(parse_movement("move 12 from 4 to 2"), [12, 4, 2]);
        assert_eq!(parse_movement("move 6 from 2 to 5"), [6, 2, 5]);
    }
}
