use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

    let lines = read_lines(INPUT_PATH)?;
    let histories = parse(lines)?;

    let (sol_a, sol_b) = solve(&histories);
    println!("{sol_a}");
    println!("{sol_b}");

    Ok(())
}

// wow https://github.com/GauravTalreja/aoc-2023/blob/main/9/src/main.rs
fn solve(histories: &Vec<Vec<i64>>) -> (i64, i64) {
    histories
        .iter()
        .map(|hist| {
            let mut seqs = vec![hist.to_owned()];
            while seqs.last().unwrap().iter().any(|&x| x != 0) {
                let seq = seqs.last().unwrap();
                let seq2: Vec<i64> = seq[..seq.len() - 1]
                    .iter()
                    .zip(seq[1..].iter())
                    .map(|(a, b)| b - a)
                    .collect();
                seqs.push(seq2);
            }
            (
                seqs.iter().rfold(0, |acc, x| acc + x.last().unwrap()),
                seqs.iter().rfold(0, |acc, x| x[0] - acc),
            )
        })
        .fold((0, 0), |acc: (i64, i64), (x, y)| (acc.0 + x, acc.1 + y))
}

fn parse(lines: io::Lines<io::BufReader<fs::File>>) -> Result<Vec<Vec<i64>>, Box<dyn Error>> {
    lines
        .map(|line| {
            line?
                .split_whitespace()
                .map(|x| Ok(x.parse::<i64>()?))
                .collect::<Result<Vec<i64>, Box<dyn Error>>>()
        })
        .collect::<Result<Vec<Vec<i64>>, Box<dyn Error>>>()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
