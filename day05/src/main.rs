fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file");
    println!(
        "The top crates are {}",
        find_top_crates(input.clone(), false).unwrap()
    );
    println!(
        "The top crates, using the same order, are {}",
        find_top_crates(input, true).unwrap()
    );
}

fn find_top_crates(s: String, use_same_order: bool) -> Option<String> {
    // Find the position of where the stacks numbers are
    let stacks_id_line_pos = s
        .lines()
        .position(|s| s.find(|c: char| c.is_ascii_digit()).is_some())?;

    // Figure out how many stacks there are
    let num_stacks = s
        .lines()
        .nth(stacks_id_line_pos)?
        .split_ascii_whitespace()
        .filter_map(|s| s.parse::<usize>().ok())
        .max()?;

    // Use the number of stack to build our vector representation of the stacks
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..num_stacks {
        stacks.push(vec![]);
    }

    // Populate the vectors
    for &line in s
        .lines()
        .filter(|l| !l.is_empty())
        .take(stacks_id_line_pos)
        .collect::<Vec<&str>>()
        .iter()
        .rev()
    {
        for stack_id in 0..num_stacks {
            if let Some(crat) = line
                .chars()
                .nth(1 + (stack_id * 4))
                .filter(|c| c.is_ascii_uppercase())
            {
                stacks[stack_id].push(crat);
            }
        }
    }

    // Apply operations
    let ops = s
        .lines()
        .filter(|l| l.contains("move"))
        .filter_map(|l| {
            let mut chunks = l
                .split_ascii_whitespace()
                .filter_map(|e| e.parse::<usize>().ok());
            Some((chunks.next()?, chunks.next()?, chunks.next()?))
        })
        .collect::<Vec<(usize, usize, usize)>>();
    for (num, src, dst) in ops {
        if use_same_order {
            // Part 2
            let end = stacks[src - 1].len();
            let start = end - num;
            let crates: Vec<char> = stacks[src - 1].drain(start..end).collect();
            for c in crates {
                stacks[dst - 1].push(c);
            }
        } else {
            // Part 1
            for _ in 0..num {
                let crat = stacks[src - 1].pop()?;
                stacks[dst - 1].push(crat);
            }
        }
    }

    // Check top crates
    let mut ans = String::new();
    for stack in stacks.iter_mut().filter_map(|v| v.pop()) {
        ans.push(stack);
    }
    Some(ans)
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = r#"
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 
    
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

    #[test]
    fn top_crates() {
        assert_eq!(
            find_top_crates(INPUT.to_string(), false),
            Some("CMZ".to_owned())
        );
    }

    #[test]
    fn top_crates_same_order() {
        assert_eq!(
            find_top_crates(INPUT.to_string(), true),
            Some("MCD".to_owned())
        );
    }
}
