use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";
    let lines = read_lines(INPUT_PATH)?;
    let cards = parse_lines(lines, Card::parse)?;

    let sol_a = solve_a(&cards);
    println!("{sol_a}");

    let sol_b = solve_b(&cards);
    println!("{sol_b}");

    Ok(())
}

fn solve_a(v: &Vec<Card>) -> u32 {
    v.iter().fold(0, |acc, card| acc + card.score)
}

fn solve_b(cards: &Vec<Card>) -> u32 {
    let mut counts = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let v = counts[i];
        let i = i + 1;
        let s = &mut counts[i..i + card.matches as usize];
        for c in s.iter_mut() {
            *c += v;
        }
    }
    counts.iter().sum()
}

struct Card {
    id: u32,
    matches: u32,
    score: u32,
}

impl Card {
    pub fn parse(s: &str) -> Result<Self, Box<dyn Error>> {
        let label_sep = s.find(':').ok_or("no label")?;
        let id: u32 = s[4..label_sep].trim().parse()?;

        let num_sep = s.find('|').ok_or("no numbers sep")?;
        let winning_nums = s[label_sep + 1..num_sep]
            .trim()
            .split_whitespace()
            .flat_map(str::parse::<u32>)
            .collect::<Vec<u32>>();
        let your_nums = s[num_sep + 1..]
            .trim()
            .split_whitespace()
            .flat_map(str::parse::<u32>);

        let matches = your_nums.filter(|x| winning_nums.contains(&x)).count() as u32;

        let base: u32 = 2;
        let score = base.pow(matches) / 2;

        Ok(Card { id, matches, score })
    }
}

fn parse_lines<F, T>(
    lines: io::Lines<io::BufReader<File>>,
    parse_fn: F,
) -> Result<Vec<T>, Box<dyn Error>>
where
    F: Fn(&str) -> Result<T, Box<dyn Error>>,
{
    lines
        .map(|line| parse_fn(&line?))
        .collect::<Result<Vec<T>, Box<dyn Error>>>()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
