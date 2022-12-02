#[derive(Copy, Clone, PartialEq)]
enum Signs {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl TryFrom<&String> for Signs {
    type Error = ();
    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase().as_str() {
            "a" | "x" => Ok(Self::Rock),
            "b" | "y" => Ok(Self::Paper),
            "c" | "z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Couldn't read input file");
    let rounds_str = input.lines().map(|l| {
        l.split_ascii_whitespace()
            .map(ToOwned::to_owned)
            .collect::<Vec<String>>()
    });
    let first_strategy = rounds_str.clone()
        .filter(|round| round.len() == 2)
        .map(|list| {
            list.iter()
                .filter_map(|element| element.try_into().ok())
                .collect::<Vec<Signs>>()
        })
        .map(|vec| (vec[0], vec[1]))
        .collect::<Vec<(Signs, Signs)>>();
    let calculate_scores = |scores: Vec<(Signs, Signs)>| -> usize {
        scores
        .iter()
        .map(|(adversary, you)| {
            let score = match (adversary, you) {
                // Draw
                (x, y) if x == y => 3,
                // Win
                (Signs::Rock, Signs::Paper)
                | (Signs::Paper, Signs::Scissors)
                | (Signs::Scissors, Signs::Rock) => 6,
                // Loss
                _ => 0,
            };
            score + *you as usize
        }).sum()
    };
    println!("Your total score for the first strategy is {}", calculate_scores(first_strategy));

    let second_strategy = rounds_str
    .filter(|round| round.len() == 2)
    .filter_map(|list| {
        Signs::try_from(&list[0]).ok().and_then(|adversary|{
            match list[1].to_ascii_lowercase().as_str() {
                // Need to lose
                "x" => match adversary {
                    Signs::Rock => Some((adversary, Signs::Scissors)),
                    Signs::Paper => Some((adversary, Signs::Rock)),
                    Signs::Scissors => Some((adversary, Signs::Paper)),
                }
                // Need to draw
                "y" => Some((adversary, adversary)),
                // Need to win
                "z" => match adversary {
                    Signs::Rock => Some((adversary, Signs::Paper)),
                    Signs::Paper => Some((adversary, Signs::Scissors)),
                    Signs::Scissors => Some((adversary, Signs::Rock)),
                }
                _ => None
            }
        })
    })
    .collect::<Vec<(Signs, Signs)>>();
    println!("Your total score for the second strategy is {}", calculate_scores(second_strategy));
}
