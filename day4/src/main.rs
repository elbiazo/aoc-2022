use itertools::Itertools;

fn main() {
    let data = include_str!("../input.txt")
        .lines()
        .map(|l| {
            l.split(",")
                .map(|p| {
                    p.split('-')
                        .map(|n| n.parse::<u32>().unwrap())
                        .collect_tuple::<(u32, u32)>()
                        .unwrap()
                })
                .collect_tuple::<(_, _)>()
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    for (p1, p2) in &data {
        if (p1.0..=p1.1).contains(&p2.0) && (p1.0..=p1.1).contains(&p2.1) {
            count += 1;
            continue;
        }

        if (p2.0..=p2.1).contains(&p1.0) && (p2.0..=p2.1).contains(&p1.1) {
            count += 1;
            continue;
        }
    }
    println!("Part 1: {}", count);

    let mut count = 0;
    for (p1, p2) in &data {
        if (p1.0..=p1.1).contains(&p2.0) || (p1.0..=p1.1).contains(&p2.1) {
            count += 1;
            continue;
        }

        if (p2.0..=p2.1).contains(&p1.0) || (p2.0..=p2.1).contains(&p1.1) {
            count += 1;
            continue;
        }
    }
    println!("Part 2: {}", count);
}
