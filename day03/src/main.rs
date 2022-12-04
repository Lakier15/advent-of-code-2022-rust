#![feature(iter_array_chunks)]

use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file");
    println!("The sum of priorities is {}", solve_part1(input.clone()));
    println!("The sum of badges priorities is {}", solve_part2(input));
}

fn solve_part1(s: String) -> usize {
    s.lines()
        .map(|s| s.as_bytes())
        .map(|bytes| {
            bytes
                .iter()
                .filter_map(|b| {
                    // Convert from ASCII code to priorities
                    match *b {
                        // A-Z
                        65..=90 => Some(b - 38),
                        // a-z
                        97..=122 => Some(b - 96),
                        _ => None,
                    }
                })
                .collect::<Vec<u8>>()
        })
        .filter_map(|priorities| {
            let mut first_half = HashSet::new();
            for b in &priorities[..priorities.len() / 2] {
                first_half.insert(*b);
            }
            let mut second_half = HashSet::new();
            for b in &priorities[priorities.len() / 2..] {
                second_half.insert(*b);
            }

            first_half.intersection(&second_half).next().cloned()
        })
        .map(|p| p as usize)
        .sum()
}

fn solve_part2(s: String) -> usize {
    s.lines()
        .map(|s| s.as_bytes())
        .map(|bytes| {
            bytes
                .iter()
                .filter_map(|b| {
                    // Convert from ASCII code to priorities
                    match *b {
                        // A-Z
                        65..=90 => Some(b - 38),
                        // a-z
                        97..=122 => Some(b - 96),
                        _ => None,
                    }
                })
                .collect::<HashSet<u8>>()
        })
        .array_chunks::<3>()
        .filter_map(|[f, s, t]| {
            let common: HashSet<u8> = f.intersection(&s).map(|ele| *ele).to_owned().collect();
            common.intersection(&t).next().cloned()
        })
        .map(|p| p as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    PmmdzqPrVvPwwTWBwg
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    ttgJtRGJQctTZtZT
    CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn example_part1() {
        assert_eq!(157, solve_part1(INPUT.to_string()));
    }

    #[test]
    fn example_part2() {
        assert_eq!(70, solve_part2(INPUT.to_string()));
    }
}
