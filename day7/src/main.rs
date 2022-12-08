use std::{
    cell::RefCell,
    fmt::Display,
    rc::{Rc, Weak},
};
const FS_MAX: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

#[derive(Default)]
pub struct Directory {
    name: String,
    parent: Option<Weak<RefCell<Directory>>>,
    children: Vec<Box<Node>>,
    // children: Box<Vec<Directory>>,
}

#[allow(dead_code)]
pub struct RegFile {
    name: String,
    parent: Weak<RefCell<Directory>>,
    size: usize,
}

enum Node {
    Reg(RegFile),
    Dir(Rc<RefCell<Directory>>),
}

trait FileSize {
    fn calculate_size(&self) -> usize;
}

impl Node {
    fn calc_size(&self) -> usize {
        match self {
            Self::Reg(file) => file.calculate_size(),
            Self::Dir(dir) => dir.as_ref().borrow().calculate_size(),
        }
    }
}

impl FileSize for RegFile {
    fn calculate_size(&self) -> usize {
        self.size
    }
}

impl FileSize for Directory {
    fn calculate_size(&self) -> usize {
        self.children
            .iter()
            .fold(0, |acc_size, node| acc_size + node.as_ref().calc_size())
    }
}

impl Directory {
    fn add_child(&mut self, node: Box<Node>) {
        self.children.push(node);
    }

    fn get_child(&self, name: &str) -> Option<&Box<Node>> {
        self.children.iter().find(|&node| match node.as_ref() {
            Node::Dir(dir) => dir.as_ref().borrow().name == name,
            Node::Reg(_) => false,
        })
    }
}

impl Display for Directory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}\nfiles: {}", self.name, self.children.len())
    }
}

pub mod parsing {
    use crate::{Directory, Node, RegFile};
    use std::{
        cell::RefCell,
        fs::File,
        io::{self, BufRead},
        rc::Rc,
    };

    pub fn parse_input(file_name: &str) -> Rc<RefCell<Directory>> {
        let file = File::open(file_name).unwrap();
        let mut lines = io::BufReader::new(file).lines();

        let root = Rc::new(RefCell::new(Directory {
            name: "/".to_string(),
            parent: None,
            children: vec![],
        }));
        let mut pwd = Rc::clone(&root);

        lines.next();

        while let Some(Ok(line)) = lines.next() {
            let tokens: Vec<&str> = line.split(' ').collect();
            if tokens[0] == "$" {
                if tokens[1] == "cd" {
                    pwd = change_dir(pwd.clone(), tokens[2]);
                }
            } else {
                // ignore "$ ls" commands,
                // if it doesn't start with '$' it's definitely an output from ls
                record_dir(pwd.clone(), &line[..])
            }
        }

        root
    }

    fn change_dir<'a>(pwd: Rc<RefCell<Directory>>, new_dir: &str) -> Rc<RefCell<Directory>> {
        if new_dir == ".." {
            return match &pwd.as_ref().borrow().parent {
                Some(parent) => parent.upgrade().expect("should be available").clone(),
                None => unreachable!(),
            };
        }

        let pnode = pwd.as_ref().borrow();

        if let Node::Dir(pdir) = &*pnode.get_child(new_dir).unwrap().as_ref() {
            Rc::clone(&pdir)
        } else {
            panic!("")
        }
    }

    fn record_dir(pwd: Rc<RefCell<Directory>>, line: &str) {
        let tokens: Vec<&str> = line.split(' ').collect();
        let node = match tokens[0] {
            "dir" => Box::new(Node::Dir(Rc::new(RefCell::new(Directory {
                name: String::from(tokens[1]),
                parent: Some(Rc::downgrade(&pwd)),
                children: Vec::new(),
            })))),
            sz => Box::new(Node::Reg(RegFile {
                name: String::from(tokens[1]),
                parent: Rc::downgrade(&pwd),
                size: sz
                    .parse()
                    .expect(&format!("{} could not be parsed into usize", sz)),
            })),
        };

        pwd.borrow_mut().add_child(node);
    }
}

fn find_smallish_dirs<'a>(root: Rc<RefCell<Directory>>) -> Vec<Rc<RefCell<Directory>>> {
    let mut ans = vec![];
    let r = root.borrow();
    let dirs = r.children.iter().filter(|&node| match **node {
        Node::Dir(_) => true,
        _ => false,
    });

    for dir in dirs {
        if let Node::Dir(dir) = &**dir {
            if dir.borrow().calculate_size() <= 100000 {
                ans.push(dir.clone());
            }
            ans.extend(find_smallish_dirs(dir.clone()));
        }
    }

    ans
}

fn find_smallest_big_enough(root: Rc<RefCell<Directory>>) -> usize {
    let occupied = root.borrow().calculate_size();
    let free_space = FS_MAX - occupied;
    let space_needed = SPACE_NEEDED - free_space;

    let r = root.borrow();

    let extract_dirs = |node: &Box<Node>| match &**node {
        Node::Dir(dir) => Some(dir.clone()),
        _ => None,
    };

    let mut queue: Vec<Rc<RefCell<Directory>>> =
        r.children.iter().filter_map(extract_dirs).collect();

    let mut min_size = FS_MAX;

    while queue.len() > 0 {
        let dir = queue.pop().unwrap();

        let dir_size = dir.borrow().calculate_size();
        if dir_size < space_needed {
            // skip children, they're all too small
            continue;
        }

        queue.extend(dir.borrow().children.iter().filter_map(extract_dirs));

        if dir_size < min_size {
            min_size = dir_size;
        }
    }

    min_size
}

fn solve1(file_name: &str) -> usize {
    let root = parsing::parse_input(file_name);
    let small_dirs = find_smallish_dirs(root);

    small_dirs
        .iter()
        .fold(0, |acc_sum, dir| acc_sum + dir.borrow().calculate_size())
}

fn solve2(file_name: &str) -> usize {
    let root = parsing::parse_input(file_name);
    find_smallest_big_enough(root)
}

fn main() {
    let ans = solve1("input.txt");
    println!("{}", ans);
    let ans = solve2("input.txt");
    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use crate::solve1;

    #[test]
    fn test_parse_input() {
        assert_eq!(solve1("test_input.txt"), 95437);
    }
}
