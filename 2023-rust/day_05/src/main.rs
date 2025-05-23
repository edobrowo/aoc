use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

    let mut lines = read_lines(INPUT_PATH)?;
    let almanac = parse(&mut lines)?;

    let sol_a = solve_a(&almanac).unwrap();
    println!("{sol_a}");

    let sol_b = solve_b(&almanac).unwrap();
    println!("{sol_b}");

    Ok(())
}

fn solve_a(almanac: &Almanac) -> Option<u64> {
    almanac
        .seeds
        .iter()
        .map(|&x| almanac.maps.iter().fold(x, |acc, m| m.get(acc)))
        .min()
}

fn rin(x: u64, a: u64, b: u64) -> bool {
    x >= a && x < (a + b)
}

fn solve_b(almanac: &Almanac) -> Option<u64> {
    // This code is demonic and awful and I should have just used or made a RangeMap but no, I chose to do this. Why? o̶̢͕̖̠͒̓n̸̜̱͓͌͝ĺ̸̯͓̝̯͗̈́͝y̴͕͆̊͝ ̷͎̓̓͝z̸̲̐͛̉͜ā̶̛̰͇̈́l̵̢̩̭̈́̎̕g̶̺̪͉͈̒̉͠o̷͚̳͆̃͗ ̸̡̘͈͌͐k̴̞̩̲͌̊̍͝n̷̳͆ǫ̶͚̏̑w̶̱̪̟͖͊̊͠s̶̲͙̦̍͋̂͂
    almanac
        .seeds
        .chunks(2)
        .map(|x| {
            let a = x[0];
            let b = x[1];
            let mut candidates = Vec::new();
            candidates.push(a);
            for (i, m) in almanac.maps.iter().enumerate() {
                for s in m.r.iter() {
                    let k1 = almanac.maps[0..i]
                        .iter()
                        .rev()
                        .fold(s.src, |acc, m| m.geti(acc));
                    let k2 = almanac.maps[0..i]
                        .iter()
                        .rev()
                        .fold(s.src + s.size, |acc, m| m.geti(acc));
                    candidates.push(k1);
                    candidates.push(k2);
                }
            }
            let res = candidates
                .iter()
                .filter(|&x| rin(*x, a, b))
                .map(|&x| almanac.maps.iter().fold(x, |acc, m| m.get(acc)))
                .min()
                .unwrap_or(u64::MAX);
            res
        })
        .min()
}

#[derive(Debug)]
struct MapSegment {
    dest: u64,
    src: u64,
    size: u64,
}

impl MapSegment {
    pub fn new(dest: u64, src: u64, size: u64) -> Self {
        MapSegment { dest, src, size }
    }

    pub fn is_key(&self, k: u64) -> bool {
        self.src <= k && k < (self.src + self.size)
    }

    pub fn is_val(&self, v: u64) -> bool {
        self.dest <= v && v < (self.dest + self.size)
    }

    pub fn get(&self, k: u64) -> u64 {
        assert!(self.is_key(k), "k must be a key");
        k - self.src + self.dest
    }

    pub fn geti(&self, v: u64) -> u64 {
        assert!(self.is_val(v), "v must be a val");
        v - self.dest + self.src
    }
}

#[derive(Debug)]
struct RangeMap {
    r: Vec<MapSegment>,
}

impl RangeMap {
    pub fn new() -> Self {
        RangeMap { r: Vec::new() }
    }

    pub fn add(&mut self, ms: MapSegment) {
        self.r.push(ms)
    }

    pub fn get(&self, k: u64) -> u64 {
        for ms in self.r.iter() {
            if ms.is_key(k) {
                return ms.get(k);
            }
        }
        k
    }

    pub fn geti(&self, v: u64) -> u64 {
        for ms in self.r.iter() {
            if ms.is_val(v) {
                return ms.geti(v);
            }
        }
        v
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<RangeMap>,
}

// https://github.com/GauravTalreja/aoc-2023/blob/main/5/src/main.rs
fn parse_rangemap(
    lines: &mut io::Lines<io::BufReader<fs::File>>,
) -> Result<RangeMap, Box<dyn Error>> {
    let mut rmap = RangeMap::new();
    let rmap_lines = lines
        .skip_while(|line| line.as_ref().is_ok_and(|line| line.is_empty()))
        .skip(1)
        .take_while(|line| line.as_ref().is_ok_and(|line| !line.is_empty()));
    for line in rmap_lines {
        let line = line?;
        let mut fields = line.split_whitespace().map(|x| x.parse::<u64>());
        let dest = fields.next().ok_or("expected field")??;
        let src = fields.next().ok_or("expected field")??;
        let size = fields.next().ok_or("expected field")??;
        rmap.add(MapSegment::new(dest, src, size));
    }
    Ok(rmap)
}

fn parse(lines: &mut io::Lines<io::BufReader<fs::File>>) -> Result<Almanac, Box<dyn Error>> {
    let seeds = lines.next().ok_or("failed to advance iter")??;
    let seeds = seeds
        .strip_prefix("seeds:")
        .ok_or("invalid prefix")?
        .split_whitespace()
        .map(|x| Ok(x.parse::<u64>()?))
        .collect::<Result<Vec<u64>, Box<dyn Error>>>()?;

    let mut maps = Vec::new();
    maps.push(parse_rangemap(lines)?);
    maps.push(parse_rangemap(lines)?);
    maps.push(parse_rangemap(lines)?);
    maps.push(parse_rangemap(lines)?);
    maps.push(parse_rangemap(lines)?);
    maps.push(parse_rangemap(lines)?);
    maps.push(parse_rangemap(lines)?);

    Ok(Almanac { seeds, maps })
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
