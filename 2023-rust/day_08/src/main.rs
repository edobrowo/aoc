use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

    let mut lines = read_lines(INPUT_PATH)?;
    let map = parse(&mut lines)?;

    // let sol_a = solve_a(&map);
    // println!("{sol_a}");

    let sol_b = solve_b(&map);
    println!("{sol_b}");

    Ok(())
}

fn solve_a(map: &Map) -> u32 {
    let mut loc = String::from("AAA");
    let dest = String::from("ZZZ");
    let mut ind = 0;
    let mut count = 0;
    while loc != dest {
        count += 1;
        let k = &loc;
        loc = match map.ins[ind] {
            Ins::Left => map.rules[k].left.clone(),
            Ins::Right => map.rules[k].right.clone(),
        };
        ind = (ind + 1) % map.ins.len();
    }
    count
}

fn gcd(a: u64, b: u64) -> u64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn solve_b(map: &Map) -> u64 {
    let mut locs: Vec<String> = Vec::new();
    let mut steps: Vec<u32> = Vec::new();
    for (rule, _) in map.rules.iter() {
        if rule.chars().last().unwrap() == 'A' {
            locs.push(rule.clone());
        }
    }
    for loc in locs.iter_mut() {
        let mut ind = 0;
        let mut count = 0;
        while !loc.ends_with('Z') {
            count += 1;
            let k = &loc;
            *loc = match map.ins[ind] {
                Ins::Left => map.rules[*k].left.clone(),
                Ins::Right => map.rules[*k].right.clone(),
            };
            ind = (ind + 1) % map.ins.len();
        }
        steps.push(count);
    }
    steps
        .iter()
        .fold(steps[0] as u64, |acc, &n| lcm(acc, n as u64))
}

#[derive(Copy, Clone, Debug)]
enum Ins {
    Left,
    Right,
}

impl Ins {
    pub fn from_char(c: &char) -> Option<Ins> {
        match c {
            'L' => Some(Ins::Left),
            'R' => Some(Ins::Right),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Rule {
    left: String,
    right: String,
}

#[derive(Debug)]
struct Map {
    ins: Vec<Ins>,
    rules: HashMap<String, Rule>,
}

fn parse(lines: &mut io::Lines<io::BufReader<fs::File>>) -> Result<Map, Box<dyn Error>> {
    let ins = lines
        .next()
        .ok_or("invalid")??
        .chars()
        .map(|c| Ins::from_char(&c))
        .collect::<Option<Vec<Ins>>>()
        .ok_or("invalid")?;

    let rules: HashMap<String, Rule> = lines
        .skip(1)
        .map(|line| {
            let line: String = line?;
            let mut line = line.split('=');
            let k = line.next().ok_or("invalid")?[0..3].to_owned();
            let mut line = line.next().ok_or("invalid")?.split(',');
            let left = line.next().ok_or("invalid")?[2..5].to_owned();
            let right = line.next().ok_or("invalid")?[1..4].to_owned();
            Ok((k, Rule { left, right }))
        })
        .collect::<Result<HashMap<String, Rule>, Box<dyn Error>>>()?;
    Ok(Map { ins, rules })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
