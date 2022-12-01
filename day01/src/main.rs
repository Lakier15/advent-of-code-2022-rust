fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file");
    let mut elfs: Vec<usize> = vec![];
    let mut calories = input.lines().peekable();
    while calories.peek().is_some() {
        elfs.push(calories
        .by_ref()
        .map_while(|s| s.parse::<usize>().ok()).sum())
    }
    elfs.sort_by(|a, b| b.cmp(a));
    elfs.truncate(3);
    println!("The elf that carries the most calories is carrying {} calories", elfs[0]);
    println!("The top 3 elfs carry together {} calories", elfs.iter().sum::<usize>());
}
