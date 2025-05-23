use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    const INPUT_PATH: &str = "input.txt";
    let lines = read_lines(INPUT_PATH).unwrap();
    let games = parse_lines(lines, parse_game).unwrap();

    let sol_a = solve_a(&games);
    println!("{sol_a}");

    let sol_b = solve_b(&games);
    println!("{sol_b}");
}

fn solve_a(games: &Vec<Game>) -> u32 {
    games.iter().fold(0, |acc, game| {
        acc + game
            .cubes
            .iter()
            .fold(true, |acc, x| {
                acc && (x.r.unwrap_or(0) <= MAX_RED
                    && x.g.unwrap_or(0) <= MAX_GREEN
                    && x.b.unwrap_or(0) <= MAX_BLUE)
            })
            .then(|| game.id)
            .unwrap_or(0)
    })
}

fn solve_b(games: &Vec<Game>) -> u32 {
    games.iter().fold(0, |acc, game| {
        let min_r = game.cubes.iter().flat_map(|x| x.r).max().unwrap();
        let min_g = game.cubes.iter().flat_map(|x| x.g).max().unwrap();
        let min_b = game.cubes.iter().flat_map(|x| x.b).max().unwrap();
        let power = min_r * min_g * min_b;
        acc + power
    })
}

#[derive(Debug)]
struct CubeSubset {
    r: Option<u32>,
    g: Option<u32>,
    b: Option<u32>,
}

impl CubeSubset {
    pub fn new() -> Self {
        CubeSubset {
            r: None,
            g: None,
            b: None,
        }
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<CubeSubset>,
}

impl Game {
    pub fn push(&mut self, cs: CubeSubset) {
        self.cubes.push(cs)
    }
}

fn parse_game(s: &str) -> Result<Game, Box<dyn Error>> {
    let label_ind = s.find(':').ok_or("missing line label")?;
    let id: u32 = s[5..label_ind].parse()?;

    let mut g: Game = Game { id, cubes: vec![] };

    let s: Vec<&str> = s[label_ind + 1..].split(';').collect();
    for draw in s {
        let mut cubes = CubeSubset::new();
        for color in draw.split(',') {
            let v: Vec<&str> = color.trim().split(' ').collect();
            let num: u32 = v[0].parse()?;
            match v[1] {
                "red" => cubes.r = Some(num),
                "green" => cubes.g = Some(num),
                "blue" => cubes.b = Some(num),
                _ => (),
            };
        }
        g.push(cubes);
    }
    Ok(g)
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

// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
