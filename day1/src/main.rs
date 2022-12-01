fn main() {
    let part1 = include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|l| {
            l.lines()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum::<u32>()
        })
        .max()
        .unwrap();
    println!("Part 1: {}", part1);

    let mut part2 = include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|l| {
            l.lines()
                .map(str::parse::<u32>)
                .map(Result::unwrap)
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();
    part2.sort();
    println!("Part 2: {}", part2.iter().rev().take(3).sum::<u32>());
}
