use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum RPS {
    Rock,
    Paper,
    Scissors,
    Lose,
    Draw,
    Win,
}

impl RPS {
    fn value(&self) -> usize {
        match *self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
            RPS::Lose => 0,
            RPS::Draw => 3,
            RPS::Win => 6,
        }
    }
    fn check(&self, player: RPS) -> usize {
        let elf = *self;
        match (&elf, &player) {
            // player loses
            (RPS::Rock, RPS::Scissors) | (RPS::Paper, RPS::Rock) | (RPS::Scissors, RPS::Paper) => 0,
            // player wins
            (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => 6,
            // Draw
            _ => 3,
        }
    }
    fn check2(&self, win_cond: RPS) -> RPS {
        let elf = *&self;
        match win_cond {
            RPS::Win => match elf {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
                _ => panic!("Error"),
            },
            RPS::Lose => match elf {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
                _ => panic!("Error"),
            },
            RPS::Draw => match elf {
                RPS::Rock => RPS::Rock,
                RPS::Paper => RPS::Paper,
                RPS::Scissors => RPS::Scissors,
                _ => panic!("Error"),
            },
            _ => panic!("Error"),
        }
    }
}
fn main() {
    let inp = include_str!("../input.txt")
        .lines()
        .map(|l| -> (RPS, RPS) {
            l.split(' ')
                .map(|v| match v {
                    "A" | "X" => RPS::Rock,
                    "B" | "Y" => RPS::Paper,
                    "C" | "Z" => RPS::Scissors,
                    _ => panic!("Invalid input {}", v),
                })
                .collect_tuple()
                .unwrap()
        })
        .map(|(elf, player)| player.value() + elf.check(player))
        .sum::<usize>();
    println!("Part 1: {}", inp);

    // I really hate this code but it was late :^)
    let inp2 = include_str!("../input.txt")
        .lines()
        .map(|l| -> (RPS, RPS) {
            l.split(' ')
                .map(|v| match v {
                    "A" => RPS::Rock,
                    "B" => RPS::Paper,
                    "C" => RPS::Scissors,
                    "X" => RPS::Lose,
                    "Y" => RPS::Draw,
                    "Z" => RPS::Win,
                    _ => panic!("Invalid input {}", v),
                })
                .collect_tuple()
                .unwrap()
        })
        .map(|(elf, rps_result)| rps_result.value() + elf.check2(rps_result).value())
        .sum::<usize>();
    println!("Part 2: {:?}", inp2);
}
