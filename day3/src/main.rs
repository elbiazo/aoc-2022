fn main() {
    let part1 = include_str!("../input.txt")
        .lines()
        .map(|l| l.split_at(l.len() / 2)) // Split string in half
        .map(|(a, b)| a.chars().into_iter().find(|c| b.contains(*c)).unwrap()) // Find matching char in haystack
        .map(|c| {
            if c.is_lowercase() {
                c as u32 - ('a' as u32) + 1
            } else {
                c as u32 - ('A' as u32) + 27
            }
        }) // Convert to number
        .sum::<u32>();

    println!("Part 1: {}", part1);

    let part2_lines = include_str!("../input.txt").lines().collect::<Vec<_>>();
    let mut groups = vec![];
    for i in (0..part2_lines.len()).step_by(3) {
        groups.push(&part2_lines[i..i + 3]);
    }

    let mut part2 = vec![];
    for group in groups {
        let group_a = group[0];
        let group_b = group[1];
        let group_c = group[2];

        for a in group_a.chars() {
            if group_b.contains(a) && group_c.contains(a) {
                part2.push(a);
                break;
            }
        }
    }
    let part2 = part2
        .into_iter()
        .map(|c| {
            if c.is_lowercase() {
                c as u32 - ('a' as u32) + 1
            } else {
                c as u32 - ('A' as u32) + 27
            }
        }) // Convert to number
        .sum::<u32>();

    println!("Part 2: {}", part2);
}
