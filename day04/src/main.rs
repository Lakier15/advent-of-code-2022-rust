use std::ops::RangeInclusive;

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file");
    let ranges = input
        .lines()
        .map(|s| s.split_terminator(",").to_owned().collect::<Vec<&str>>())
        .map(|v| {
            v.iter()
                .filter_map(|ele| {
                    let mut iter = ele.split_terminator("-");
                    let start: usize = iter.next()?.to_string().parse().ok()?;
                    let end: usize = iter.next()?.to_string().parse().ok()?;
                    Some(start..=end)
                })
                .collect::<Vec<RangeInclusive<usize>>>()
        });
    let fully_contained = ranges.clone()
        .filter(|v| {
            let first_contains_second = v[0].contains(v[1].start()) && v[0].contains(v[1].end());
            let second_contains_first = v[1].contains(v[0].start()) && v[1].contains(v[0].end());
            first_contains_second || second_contains_first
        });
    println!("There are {} fully contained pairs", fully_contained.count());
    let partially_contained = ranges.filter(|v|{
        let first_contains_second = v[0].contains(v[1].start()) || v[0].contains(v[1].end());
        let second_contains_first = v[1].contains(v[0].start()) || v[1].contains(v[0].end());
        first_contains_second || second_contains_first
    });
    println!("There are {} partially contained pairs", partially_contained.count());
}
