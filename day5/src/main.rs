use std::collections::HashMap;

use regex::Regex;
fn get_data() -> (HashMap<usize, Vec<String>>, Vec<Vec<usize>>, usize) {
    let mut crates_map = HashMap::<usize, Vec<String>>::new();

    let unparsed_data = include_str!("../input.txt")
        .split("\n\n")
        .collect::<Vec<&str>>();
    let mut unparsed_crates = unparsed_data[0].lines().collect::<Vec<&str>>();
    let unparsed_inst = unparsed_data[1].lines().collect::<Vec<&str>>();

    let key_re = Regex::new(r"\d+").unwrap();
    let crate_key = key_re
        .find_iter(unparsed_crates.pop().unwrap())
        .into_iter()
        .map(|k| k.as_str().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let crates = unparsed_crates
        .into_iter()
        .map(|l| {
            l.chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .map(|c| {
                    c.iter()
                        .collect::<String>()
                        .trim()
                        .to_string()
                        .replace("[", "")
                        .replace("]", "")
                })
                .collect::<Vec<String>>()
        })
        .rev()
        .collect::<Vec<_>>();

    for c in crates {
        for (i, v) in c.iter().enumerate() {
            if v.len() != 0 {
                crates_map
                    .entry(crate_key[i])
                    .or_insert(Vec::new())
                    .push(v.to_string());
            }
        }
    }
    let inst_re = Regex::new(r"\d+").unwrap();
    let insts = unparsed_inst
        .into_iter()
        .map(|l| {
            inst_re
                .find_iter(l)
                .into_iter()
                .map(|k| k.as_str().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<_>>>();
    (crates_map, insts, crate_key.len())
}

fn main() {
    let (mut crates_map, insts, max_index) = get_data();
    for inst in insts {
        for _ in 1..=inst[0] {
            let tmp = crates_map.get_mut(&inst[1]).unwrap().pop().unwrap();
            crates_map.get_mut(&inst[2]).unwrap().push(tmp);
        }
    }

    let mut result: Vec<char> = vec![' '; max_index];
    for (k, v) in crates_map {
        result[k - 1] = v[v.len() - 1].chars().next().unwrap();
    }
    let result = result.into_iter().collect::<String>();
    println!("Part1: {}", result);

    let (mut crates_map, insts, max_index) = get_data();

    for inst in insts {
        let mut tmp = vec![];
        for _ in 1..=inst[0] {
            tmp.push(crates_map.get_mut(&inst[1]).unwrap().pop().unwrap());
        }

        tmp = tmp.into_iter().rev().collect::<Vec<_>>();

        for t in tmp {
            crates_map.get_mut(&inst[2]).unwrap().push(t);
        }
    }

    let mut result: Vec<char> = vec![' '; max_index];
    for (k, v) in crates_map {
        result[k - 1] = v[v.len() - 1].chars().next().unwrap();
    }
    let result = result.into_iter().collect::<String>();
    println!("Part1: {}", result);
}
