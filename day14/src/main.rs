use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, Clone, Copy)]
pub enum Block {
    Air,
    Rock,
    Sand,
    Origin,
}

type Coord = (usize, usize);

#[derive(Debug, Clone)]
pub struct Grid {
    blocks: Vec<VecDeque<Block>>,
    left_width: usize,
    right_width: usize,
    origin: Coord,
}

impl Default for Block {
    fn default() -> Self {
        Self::Air
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.show())
    }
}

impl Grid {
    pub fn new() -> Grid {
        Grid {
            blocks: vec![VecDeque::from_iter([Block::Origin])],
            left_width: 0,
            right_width: 1,
            origin: (500, 0),
        }
    }

    /// translate a x coordinate into an index
    /// return `None` if such an index does not exist
    fn x_coord_index(&self, x: usize) -> Option<usize> {
        if x < self.origin.0 - self.left_width {
            // would cause overflow
            return None;
        }
        let index = x - (self.origin.0 - self.left_width);
        if index >= self.left_width + self.right_width {
            return None;
        }

        Some(index)
    }

    pub fn show(&self) -> String {
        self.blocks
            .iter()
            .map(|row| {
                row.iter()
                    .map(|block| match block {
                        Block::Air => ' ',
                        Block::Rock => '#',
                        Block::Sand => 'o',
                        Block::Origin => '+',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn place(&mut self, block: Block, at: Coord) {
        let i = match self.x_coord_index(at.0) {
            Some(i) => i,
            None => {
                self.extend_hori_to(at.0);
                self.x_coord_index(at.0).unwrap()
            }
        };

        if at.1 >= self.blocks.len() {
            self.extend_vert_to(at.1)
        }
        let j = at.1;

        self.blocks[j][i] = block;
    }

    fn extend_hori_to(&mut self, x: usize) {
        if x < self.origin.0 {
            self.extend_left_to(x)
        } else {
            self.extend_right_to(x)
        }
    }

    fn extend_vert_to(&mut self, y: usize) {
        self.blocks.reserve(y - self.blocks.len() + 1);
        for _ in self.blocks.len()..y + 1 {
            // TODO better way to initialize VecDeque?
            // TODO is it better to init row outside loop and use copy magic?
            let row = VecDeque::from_iter(vec![Block::Air; self.width()]);
            self.blocks.push(row)
        }
    }

    fn width(&self) -> usize {
        self.left_width + self.right_width
    }

    fn extend_left_to(&mut self, x: usize) {
        let to_add = (self.origin.0 - x) - self.left_width;
        for row in self.blocks.iter_mut() {
            for _ in 0..to_add {
                row.push_front(Block::Air);
            }
        }

        self.left_width += to_add
    }

    fn extend_right_to(&mut self, x: usize) {
        let to_add = (x - self.origin.0 + 1) - self.right_width;
        for row in self.blocks.iter_mut() {
            row.extend(vec![Block::Air; to_add]);
        }

        self.right_width += to_add;
    }
}

mod parser {
    use crate::{Block, Coord, Grid};
    use std::{
        fs::File,
        io::{BufRead, BufReader},
    };

    pub fn parse_input(fname: &str) -> Grid {
        let file = File::open(fname).unwrap();
        let lines = BufReader::new(file).lines();
        let mut grid = Grid::new();

        for line in lines {
            let line = line.unwrap();
            let vertices: Vec<Coord> = line.split(" -> ").map(|coord| parse_coord(coord)).collect();

            for v in vertices.windows(2) {
                let start = v[0];
                let end = v[1];
                for coord in get_line_coords(start, end) {
                    grid.place(Block::Rock, coord);
                }
            }
        }

        grid
    }

    fn parse_coord(line: &str) -> Coord {
        let mut split = line.split(',');
        let x = split.next().unwrap().parse().unwrap();
        let y = split.next().unwrap().parse().unwrap();
        (x, y)
    }

    fn get_line_coords(start: Coord, end: Coord) -> Vec<Coord> {
        if start.0 == end.0 {
            let fr = if start.1 < end.1 { start.1 } else { end.1 };
            let to = if start.1 < end.1 { end.1 } else { start.1 };
            (fr..to + 1).map(|y| (start.0, y)).collect()
        } else {
            let fr = if start.0 < end.0 { start.0 } else { end.0 };
            let to = if start.0 < end.0 { end.0 } else { start.0 };
            (fr..to + 1).map(|x| (x, start.1)).collect()
        }
    }
}

mod simulation {
    use std::collections::VecDeque;

    use crate::{Block, Coord, Grid};

    enum SandState {
        Rest(Coord),
        Move(Coord),
        Abyss,
    }

    impl Grid {
        pub fn start_simulate(&mut self) -> u32 {
            let mut rounds = 0;
            while let SandState::Rest(_) = self.simulate_one_sand() {
                rounds += 1;
            }
            rounds
        }

        pub fn start_simulate_2(&mut self) -> u32 {
            self.add_floor();
            let mut rounds = 0;
            while let SandState::Rest(at) = self.simulate_one_sand() {
                rounds += 1;
                if at == self.origin {
                    return rounds;
                }
            }

            println!("{}", self);
            panic!("reached Abyss!");
        }

        fn simulate_one_sand(&mut self) -> SandState {
            let mut sand = SandState::Move(self.origin);
            loop {
                sand = match sand {
                    SandState::Move(pos) => self.next_sand_pos(pos),
                    SandState::Rest(pos) => {
                        self.place(Block::Sand, pos);
                        return SandState::Rest(pos);
                    }
                    SandState::Abyss => return SandState::Abyss,
                }
            }
        }

        fn next_sand_pos(&self, curr_pos: Coord) -> SandState {
            let (x0, y0) = curr_pos;
            for (x, y) in [(x0, y0 + 1), (x0 - 1, y0 + 1), (x0 + 1, y0 + 1)] {
                let db = self.get((x, y));
                match db {
                    Some(Block::Air) => return SandState::Move((x, y)),
                    None => return SandState::Abyss,
                    Some(_) => (),
                }
            }

            SandState::Rest((x0, y0))
        }

        // return None if out of bounds
        fn get(&self, coord: Coord) -> Option<Block> {
            let i = match self.x_coord_index(coord.0) {
                Some(i) => i,
                None => return None,
            };

            if self.blocks.len() <= coord.1 {
                return None;
            }

            Some(self.blocks[coord.1][i])
        }

        fn add_floor(&mut self) {
            // theoretically, the maximum width of the floor can be calculated with
            //      width_max = h * 2
            // where h is the distance between origin and floor

            let h = self.blocks.len() + 2;

            self.extend_left_to(self.origin.0 - h);
            self.extend_right_to(self.origin.0 + h); // FIXME

            self.extend_vert_to(self.blocks.len());

            let row = VecDeque::from_iter(vec![Block::Rock; self.width()]);
            self.blocks.push(row);
        }
    }
}

fn main() {
    let mut grid = parser::parse_input("input.txt");
    let mut grid2 = grid.clone();
    let ans = grid.start_simulate();

    let ans2 = grid2.start_simulate_2();

    println!("{}", grid);
    println!("{}", grid2);

    println!("{}", ans);
    println!("{}", ans2)
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_input;
    // use crate::simulation;

    #[test]
    fn test_parse_input() {
        let grid = parse_input("test_input.txt");
        let expected = r#"      +   
          
          
          
    #   ##
    #   # 
  ###   # 
        # 
        # 
######### "#;
        assert_eq!(format!("{}", grid), format!("{}", expected))
    }

    #[test]
    fn test_simulate() {
        let mut grid = parse_input("test_input.txt");
        let ans = grid.start_simulate();

        assert_eq!(ans, 24);
    }

    #[test]
    fn test_simulate2() {
        let mut grid = parse_input("test_input.txt");
        let ans = grid.start_simulate_2();

        assert_eq!(ans, 93);
    }
}
