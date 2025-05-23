use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path;

fn main() -> Result<(), Box<dyn Error>> {
    const INPUT_PATH: &str = "input.txt";

    let lines = read_lines(INPUT_PATH)?;
    let mut hands = parse(lines)?;

    let sol_a = solve_a(&mut hands);
    println!("{sol_a}");

    let sol_b = solve_b(&mut hands);
    println!("{sol_b}");

    Ok(())
}

fn solve_a(hands: &mut Vec<Hand>) -> u32 {
    hands.sort();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x.bid * (i as u32 + 1))
}

fn solve_b(hands: &mut Vec<Hand>) -> u32 {
    // :shrug:
    for h in hands.iter_mut() {
        for c in h.cards.iter_mut() {
            if *c == Card::Jack {
                *c = Card::Joker;
            }
        }
        if h.cards.contains(&Card::Joker) {
            let jcount = h.cards.iter().filter(|&x| *x == Card::Joker).count();
            h.kind = match jcount {
                4 => HandKind::FiveOfAKind,
                3 => match h.kind {
                    HandKind::FullHouse => HandKind::FiveOfAKind,
                    _ => HandKind::FourOfAKind,
                },
                2 => match h.kind {
                    HandKind::FullHouse => HandKind::FiveOfAKind,
                    HandKind::TwoPair => HandKind::FourOfAKind,
                    HandKind::OnePair => HandKind::ThreeOfAKind,
                    _ => h.kind.clone(),
                },
                1 => match h.kind {
                    HandKind::FourOfAKind => HandKind::FiveOfAKind,
                    HandKind::FullHouse => HandKind::FourOfAKind,
                    HandKind::ThreeOfAKind => HandKind::FourOfAKind,
                    HandKind::TwoPair => HandKind::FullHouse,
                    HandKind::OnePair => HandKind::ThreeOfAKind,
                    HandKind::HighCard => HandKind::OnePair,
                    _ => h.kind.clone(),
                },
                _ => h.kind.clone(),
            }
        }
    }
    hands.sort();
    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x.bid * (i as u32 + 1))
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    pub fn from_str(cards: &str, counts: &mut HashMap<char, u32>) -> HandKind {
        counts.clear();
        for c in cards.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        use HandKind::*;
        let mut hk = HighCard;
        for (_, &v) in counts.iter() {
            match v {
                5 => hk = FiveOfAKind,
                4 => hk = FourOfAKind,
                3 => {
                    if hk == OnePair {
                        hk = FullHouse
                    } else {
                        hk = ThreeOfAKind
                    }
                }
                2 => {
                    if hk == OnePair {
                        hk = TwoPair
                    } else if hk == ThreeOfAKind {
                        hk = FullHouse
                    } else {
                        hk = OnePair
                    }
                }
                _ => (),
            }
        }
        hk
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    pub fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::Jack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Hand {
    kind: HandKind,
    cards: [Card; 5],
    bid: u32,
}

fn parse(lines: io::Lines<io::BufReader<fs::File>>) -> Result<Vec<Hand>, Box<dyn Error>> {
    let mut hands: Vec<Hand> = Vec::new();
    let mut counts: HashMap<char, u32> = HashMap::new();
    for line in lines {
        let line = line?;
        let mut line = line.split_whitespace();
        let card_str = line.next().ok_or("invalid line")?;
        let cards: [Card; 5] = [
            Card::from_char(card_str.chars().nth(0).ok_or("invalid")?).ok_or("invalid")?,
            Card::from_char(card_str.chars().nth(1).ok_or("invalid")?).ok_or("invalid")?,
            Card::from_char(card_str.chars().nth(2).ok_or("invalid")?).ok_or("invalid")?,
            Card::from_char(card_str.chars().nth(3).ok_or("invalid")?).ok_or("invalid")?,
            Card::from_char(card_str.chars().nth(4).ok_or("invalid")?).ok_or("invalid")?,
        ];
        let bid = line.next().ok_or("invald line")?.parse::<u32>()?;
        let kind = HandKind::from_str(&card_str, &mut counts);
        hands.push(Hand { cards, bid, kind });
    }

    Ok(hands)
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
