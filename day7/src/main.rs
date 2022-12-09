use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
}

#[derive(Debug, Clone)]
struct File {
    name: String,
    size: usize,
}
#[derive(Debug, Clone)]
enum Inst {
    Ls,
    Cd(String),
}
#[derive(Debug, Clone)]
enum Code {
    Inst(Inst),
    File(File),
    Dir(Dir),
}
struct Term {
    root: NodeRef,
    free_mem: Vec<usize>,
}

type NodeRef = Rc<RefCell<Node>>;

#[derive(Default)]
struct Node {
    size: usize,
    files: Vec<File>,
    parent: Option<NodeRef>,
    childs: HashMap<String, NodeRef>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Node")
            .field("size", &self.size)
            // .field("files", &self.files)
            .field("childs", &self.childs)
            .finish()
    }
}
impl Term {
    fn new() -> Self {
        Self {
            root: Rc::new(RefCell::new(Node::default())),
            free_mem: vec![],
        }
    }
    fn run(&mut self, input: &str) {
        let codes = self.parse(input);
        let root = Rc::new(RefCell::new(Node::default()));
        let mut node = Rc::clone(&root);

        for code in codes {
            match code {
                Code::Dir(dir) => {
                    // let entry = node.borrow_mut().childs.entry(dir.name).or_default();
                    let entry = node
                        .borrow_mut()
                        .childs
                        .entry(dir.name)
                        .or_default()
                        .clone();
                    entry.borrow_mut().parent = Some(Rc::clone(&node));
                }

                Code::File(file) => {
                    node.borrow_mut().size += file.size;
                    node.borrow_mut().files.push(file);
                }
                Code::Inst(inst) => match inst {
                    Inst::Ls => {}
                    Inst::Cd(dir) => match dir.as_str() {
                        "/" => {
                            node = Rc::clone(&root);
                        }
                        ".." => {
                            let parent = node.borrow().parent.clone().unwrap();
                            node = parent;
                        }
                        _ => {
                            let child = node.borrow_mut().childs.entry(dir).or_default().clone();
                            node = child;
                        }
                    },
                },
            }
        }

        self.root = root;
        self.update_total_size(self.root.clone());
    }

    fn parse(&self, input: &str) -> Vec<Code> {
        let mut codes: Vec<Code> = Vec::new();
        for line in input.lines() {
            if line.chars().nth(0).unwrap() == '$' {
                codes.push(Code::Inst(self.parse_inst(&line[1..])));
            } else {
                if &line[0..3] == "dir" {
                    codes.push(Code::Dir(self.parse_dir(&line)));
                    continue;
                } else {
                    codes.push(Code::File(self.parse_file(&line)));
                }
            }
        }
        codes
    }
    fn parse_file(&self, dir: &str) -> File {
        let dir = dir.to_string();
        let (size, name) = dir.trim().split(" ").collect_tuple().unwrap();
        File {
            name: name.to_string(),
            size: size.parse::<usize>().unwrap(),
        }
    }
    fn parse_dir(&self, dir: &str) -> Dir {
        Dir {
            name: dir.trim().replace(" ", "")[3..].to_string(),
        }
    }

    fn parse_inst(&self, inst: &str) -> Inst {
        let inst = inst.trim().replace(" ", "");
        if &inst[0..2] == "cd" {
            Inst::Cd(inst[2..].to_string())
        } else if &inst[0..2] == "ls" {
            Inst::Ls
        } else {
            panic!("Unknown instruction: {}", inst)
        }
    }
    fn update_total_size(&self, node: NodeRef) -> usize {
        let mut total_size = node.borrow().size;

        for child in node.borrow().childs.clone() {
            total_size += self.update_total_size(Rc::clone(&child.1));
        }

        node.borrow_mut().size = total_size;
        total_size
    }

    fn part1(&self, node: NodeRef, limit: usize) -> usize {
        let mut cur_size = match node.borrow().size {
            size if size < limit => size,
            _ => 0,
        };
        for child in node.borrow().childs.clone() {
            cur_size += self.part1(Rc::clone(&child.1), limit);
        }
        cur_size
    }
    fn part2(&mut self, node: NodeRef, min_space: usize) {
        let cur_size = node.borrow().size;

        if cur_size >= min_space {
            self.free_mem.push(cur_size);
        }

        for child in node.borrow().childs.clone() {
            self.part2(Rc::clone(&child.1), min_space);
        }
    }
}

fn main() {
    let mut term = Term::new();
    let input = include_str!("../input.txt");

    term.run(input);
    let part1 = term.part1(term.root.clone(), 100000);

    println!("part1: {}", part1);
    let total_space = 70000000_usize;
    let used_space = term.root.borrow().size;
    let free_space = total_space.checked_sub(used_space).unwrap();
    let needed_free_space = 30000000_usize;
    let minimum_space_to_free = needed_free_space.checked_sub(free_space).unwrap();
    term.part2(term.root.clone(), minimum_space_to_free);

    println!("part2: {}", term.free_mem.iter().min().unwrap());
}
