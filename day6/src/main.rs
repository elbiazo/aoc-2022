use std::collections::HashSet;
fn solve(marker: usize) -> usize {
    include_bytes!("../input.txt")
        .windows(marker)
        .position(|w| w.iter().collect::<HashSet<&u8>>().len() == marker)
        .map(|p| p + marker)
        .unwrap()
}
fn main() {
    let part1 = solve(4);
    println!("Part 1: {}", part1);

    let part2 = solve(14);
    println!("Part 2: {}", part2);
}
