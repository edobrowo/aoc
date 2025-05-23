use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Read;

// ugly >:(
fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

    let mut data = String::new();
    let mut file = File::open(INPUT_PATH)?;
    file.read_to_string(&mut data)?;
    let lines = data.lines().map(str::as_bytes).collect::<Vec<_>>();

    // https://github.com/AxlLind/AdventOfCode2023/blob/main/src/bin/03.rs
    let mut parts: Vec<Part> = vec![];
    let mut gears: HashMap<Symbol, Vec<u32>> = HashMap::new();

    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;
        while col < line.len() {
            if !line[col].is_ascii_digit() {
                col = col + 1;
                continue;
            }
            let s = col;
            let mut sym: Option<Symbol> = None;
            while col < line.len() && line[col].is_ascii_digit() {
                let dsym = get_sym(&lines, row, col);
                if dsym.is_some() {
                    sym = dsym;
                }
                col = col + 1;
            }
            let id = *&line[s..col]
                .iter()
                .fold(0 as u32, |acc, d| acc * 10 + ((*d - b'0') as u32));
            if sym.is_some() {
                let sym = sym.clone().unwrap();
                gears.entry(sym).or_insert(Vec::new()).push(id);
            }
            parts.push(Part { id, sym });
        }
    }

    let sol_a = solve_a(&parts);
    println!("{sol_a}");

    let sol_b = solve_b(&gears);
    println!("{sol_b}");

    Ok(())
}

fn solve_a(p: &Vec<Part>) -> u32 {
    p.iter()
        .fold(0, |acc, x| acc + x.sym.is_some().then(|| x.id).unwrap_or(0))
}

fn solve_b(g: &HashMap<Symbol, Vec<u32>>) -> u32 {
    g.iter()
        .filter(|&(k, v)| k.ch == b'*' && v.len() == 2)
        .fold(0, |acc, (_, v)| acc + v[0] * v[1])
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
struct Symbol {
    ch: u8,
    row: usize,
    col: usize,
}

struct Part {
    id: u32,
    sym: Option<Symbol>,
}

fn get_sym(lines: &[&[u8]], row: usize, col: usize) -> Option<Symbol> {
    for (drow, dcol) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        let drow = row as i32 + drow;
        let dcol = col as i32 + dcol;
        if drow < 0 || drow >= lines.len() as i32 || dcol < 0 || dcol >= lines[0].len() as i32 {
            continue;
        }
        let drow = drow as usize;
        let dcol = dcol as usize;
        if lines[drow][dcol] != b'.' && !lines[drow][dcol].is_ascii_digit() {
            return Some(Symbol {
                ch: lines[drow][dcol],
                row: drow,
                col: dcol,
            });
        }
    }
    None
}
