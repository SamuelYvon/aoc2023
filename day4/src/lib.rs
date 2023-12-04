use regex::{Match, Regex};
use utils::lines_of_file;
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;
use std::usize;
use utils::Problem;

pub struct Day4();

impl Problem for Day4 {
    fn get_part1(&self) -> fn(bool) -> () {
        part1::run
    }

    fn get_part2(&self) -> fn(bool) -> () {
        part2::run
    }
}

pub const DAY_4: Day4 = Day4();

fn filename(debug: bool) -> &'static str {
    match debug {
        true => "files/day4.example",
        false => "files/day4.puzzle",
    }
}

#[derive(Debug)]
struct Card {
    number: usize,
    winning: HashMap<u32, usize>,
    gotten: HashMap<u32, usize>,
}

impl Card {
    pub fn new(number: usize, winning: HashMap<u32, usize>, gotten: HashMap<u32, usize>) -> Self {
        Card {
            number,
            winning,
            gotten,
        }
    }

    pub fn matches(&self) -> usize {
        let mut matches = 0;
        self.gotten.iter().for_each(|(key, val)| {
            if self.winning.contains_key(key) {
                matches += val
            }
        });
        matches
    }

    pub fn score(&self) -> u32 {
        let sum = self.matches();
        if sum > 0 {
            2_u32.pow((sum - 1) as u32)
        } else {
            0
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let score = self.score();
        f.write_fmt(format_args!("score: {score}"))
    }
}

fn into_hashmap(m: &Option<Match>, map: &mut HashMap<u32, usize>) {
    let m = m.unwrap().as_str();
    let nums = m
        .trim()
        .split(" ")
        .filter_map(|num_part| u32::from_str(num_part.trim()).ok())
        .collect::<Vec<_>>();

    nums.into_iter().for_each(|num| {
        let entry = map.entry(num).or_insert(0);
        *entry += 1;
    });
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut regex =
            Regex::new(r"(?m)Card *(?P<number>\d*): (?P<winning>( *\d* *)*) | *(?P<gotten>(\d* *)*)").unwrap();
        let captures = regex.captures_iter(s);

        let mut number: usize = 0;
        let mut winning = HashMap::<u32, usize>::new();
        let mut gotten = HashMap::<u32, usize>::new();

        captures.for_each(|c| {
            let winning_cap = c.name("winning");
            let gotten_cap = c.name("gotten");
            let number_cap = c.name("number");

            if winning_cap.is_some() {
                into_hashmap(&winning_cap, &mut winning);
            }

            if gotten_cap.is_some() {
                into_hashmap(&gotten_cap, &mut gotten);
            }

            if number_cap.is_some() {
                number = usize::from_str(number_cap.unwrap().as_str().trim()).unwrap();
            }
        });

        Ok(Card::new(number, winning, gotten))
    }
}


mod part2 {
    use utils::lines_of_file;
    use super::*;

    fn solve(file: &str) -> u32 {
        let lines = lines_of_file(file).unwrap();
        let cards = lines
            .iter()
            .map(|line| Card::from_str(line).unwrap())
            .collect::<Vec<_>>();

        let mut number_of_each = HashMap::<usize, usize>::new();

        for card in cards.iter() {
            // Count this card as an entry, we have it!
            let entries = number_of_each.entry(card.number).or_insert(0);
            *entries += 1;
            let entries = *entries;

            let n = card.matches();

            for i in card.number + 1..=card.number + n {
                *number_of_each.entry(i).or_insert(0) += entries;
            }
        }

        number_of_each.values().sum::<usize>() as u32
    }

    pub fn run(debug: bool) {
        let filename = filename(debug);
        let score = solve(filename);
        println!("The score is {score}");
    }
}

mod part1 {
    use crate::*;

    fn solve(file: &str) -> u32 {
        let lines = lines_of_file(file).unwrap();
        let cards = lines
            .iter()
            .map(|line| Card::from_str(line).unwrap())
            .collect::<Vec<_>>();

        for card in &cards {
            println!("{card}")
        }

        cards.iter().map(|c| c.score()).sum::<u32>()
    }

    pub fn run(debug: bool) {
        let filename = filename(debug);
        let score = solve(filename);

        println!("The score is {score}");
    }
}
