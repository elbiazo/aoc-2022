use std::fmt;

struct Quadcopter {
    trees: Vec<Vec<u32>>,
}

impl fmt::Debug for Quadcopter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tree_fmt = String::new();
        for row in &self.trees {
            for c in row {
                tree_fmt.push_str(&format!("{} ", c));
            }
            tree_fmt.push_str("\n");
        }
        writeln!(f, "{}", tree_fmt)
    }
}

#[derive(Debug)]
struct Tree {
    cord: (u32, u32),
    is_edge: bool,
    is_visible: bool,
}

impl Quadcopter {
    fn new(data: &str) -> Self {
        Quadcopter {
            trees: data
                .lines()
                .map(|l| {
                    l.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>(),
        }
    }

    fn part1(&self) -> usize {
        let mut vis = vec![vec![false; self.trees[0].len()]; self.trees.len()];
        for i in 0..self.trees.len() {
            for j in 0..self.trees[i].len() {
                vis[i][j] = self.visible(i, j);
            }
        }

        vis.into_iter().flatten().filter(|&v| v).count()
    }

    fn part2(&self) -> usize {
        let mut cnt = vec![];
        for i in 0..self.trees.len() {
            for j in 0..self.trees[i].len() {
                cnt.push(self.count(i, j));
            }
        }
        cnt.into_iter().max().unwrap()
    }

    fn count(&self, x: usize, y: usize) -> usize {
        let mut left = 0;
        let cur = self.trees[x][y];

        // check left
        for j in (0..y).rev() {
            left += 1;
            if self.trees[x][j] >= cur {
                break;
            }
        }

        let mut right = 0;
        // check right
        for j in (y + 1..self.trees[x].len()) {
            right += 1;
            if self.trees[x][j] >= cur {
                break;
            }
        }

        // check up
        let mut up = 0;
        for i in (0..x).rev() {
            up += 1;
            if self.trees[i][y] >= cur {
                break;
            }
        }

        // check down
        let mut down = 0;
        for i in (x + 1)..self.trees.len() {
            down += 1;
            if self.trees[i][y] >= cur {
                break;
            }
        }

        left * right * up * down
    }

    fn visible(&self, x: usize, y: usize) -> bool {
        // is edge
        if x == 0 || y == 0 {
            return true;
        } else {
            // check left
            let cur = self.trees[x][y];
            for j in 0..=y {
                // it is visible
                if j == y {
                    return true;
                }

                if self.trees[x][j] >= cur {
                    break;
                }
            }

            // check right
            for j in (y..self.trees[x].len()).rev() {
                if j == y {
                    return true;
                }

                if self.trees[x][j] >= cur {
                    break;
                }
            }

            // check up
            for i in 0..=x {
                if i == x {
                    return true;
                }

                if self.trees[i][y] >= cur {
                    break;
                }
            }

            // check down
            for i in (x..self.trees.len()).rev() {
                if i == x {
                    return true;
                }

                if self.trees[i][y] >= cur {
                    break;
                }
            }
        }

        false
    }
}
fn main() {
    let quad = Quadcopter::new(include_str!("../input.txt"));
    println!("part 1: {}", quad.part1());
    println!("part 2: {}", quad.part2());
}
