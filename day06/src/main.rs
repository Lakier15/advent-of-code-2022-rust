use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::Hash;

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file");
    println!(
        "{} need to be processed to encounter first start-of-packet marker",
        find_unique_seq_pos(input.clone(), 4).unwrap()
    );
    println!(
        "{} need to be processed to encounter first start-of-message marker",
        find_unique_seq_pos(input, 14).unwrap()
    );
}

fn find_unique_seq_pos(s: String, num_unique: usize) -> Option<usize> {
    let mut seq = VecDeque::with_capacity(num_unique);
    for (i, c) in s.chars().enumerate() {
        seq.push_back(c);
        if seq.len() == num_unique {
            if has_unique_elements(&seq) {
                return Some(i + 1);
            }
            seq.pop_front();
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT2: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT3: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT4: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1() {
        assert_eq!(Some(5), find_unique_seq_pos(INPUT1.to_owned(), 4));
        assert_eq!(Some(6), find_unique_seq_pos(INPUT2.to_owned(), 4));
        assert_eq!(Some(10), find_unique_seq_pos(INPUT3.to_owned(), 4));
        assert_eq!(Some(11), find_unique_seq_pos(INPUT4.to_owned(), 4));
    }
}
