use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn detect_unique(msg: &[u8], num_unique: usize) -> usize {
    let mut seen: HashSet<u8> = HashSet::new();
    let mut window = VecDeque::with_capacity(num_unique);

    for (i, chr) in msg.iter().enumerate() {
        window.push_front(chr);
        if seen.contains(chr) {
            while let Some(to_remove) = window.pop_back() {
                seen.remove(to_remove);
                if to_remove == chr {
                    break;
                }
            }
        } else if window.len() == num_unique {
            return i + 1;
        };
        seen.insert(*chr);
    }

    return 0;
}

fn detect_packet(msg: &[u8]) -> usize {
    detect_unique(msg, 4)
}

fn detect_message(msg: &[u8]) -> usize {
    detect_unique(msg, 14)
}

fn main() {
    let msg = fs::read("input.txt").unwrap();
    let ans1 = detect_packet(&msg[..]);
    println!("{ans1}");
    let ans2 = detect_message(&msg[..]);
    println!("{ans2}");
}

#[cfg(test)]
mod tests {
    use crate::{detect_message, detect_packet};

    #[test]
    fn test_solve() {
        struct Test {
            input: Vec<u8>,
            output: usize,
        }
        let tests = [
            Test {
                input: "bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes().to_vec(),
                output: 5,
            },
            Test {
                input: "nppdvjthqldpwncqszvftbrmjlhg".as_bytes().to_vec(),
                output: 6,
            },
            Test {
                input: "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes().to_vec(),
                output: 10,
            },
            Test {
                input: "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes().to_vec(),
                output: 11,
            },
        ];

        for t in tests {
            let ans = detect_packet(&t.input);
            if ans != t.output {
                panic!(
                    "with input {}\nexpected: {}, got {}",
                    String::from_utf8(t.input).unwrap(),
                    t.output,
                    ans
                );
            }
        }
    }

    #[test]
    fn test_detect_message() {
        struct Test(&'static [u8], usize);
        let tests = [
            Test("mjqjpqmgbljsphdztnvjfqwrcgsmlb".as_bytes(), 19),
            Test("bvwbjplbgvbhsrlpgdmjqwftvncz".as_bytes(), 23),
            Test("nppdvjthqldpwncqszvftbrmjlhg".as_bytes(), 23),
            Test("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".as_bytes(), 29),
            Test("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".as_bytes(), 26),
        ];

        for t in tests {
            let ans = detect_message(t.0);
            assert_eq!(ans, t.1);
        }
    }
}
