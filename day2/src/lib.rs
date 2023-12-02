use std::cmp::max;
use regex::Regex;
use std::ops::Deref;
use std::str::FromStr;
use utils::lines_of_file;

#[derive(Debug, PartialEq)]
struct Pick {
    red: u32,
    blue: u32,
    green: u32,
}

impl Pick {
    fn power(&self) -> u32 {
        self.red * self.blue * self.green
    }
}

#[derive(Debug)]
struct Game {
    id: u32,
    picks: Vec<Pick>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let be = s.find(':').expect("No beginning of rounds");

        // Get the game id
        let id: String = s[..be]
            .chars()
            .filter(|c| c.is_numeric())
            .collect();
        let id = u32::from_str(&id).unwrap_or_else(|_| panic!("Invalid id: got {s}"));

        let splits = s[be..].split(';');
        let color_regex = Regex::new(r"(\d+) (red|green|blue)").unwrap();

        let mut picks: Vec<Pick> = vec![];

        for split in splits {
            let mut red = 0_u32;
            let mut green = 0_u32;
            let mut blue = 0_u32;

            let haystack = String::from(split).replace(':', "");

            color_regex
                .captures_iter(haystack.trim())
                .for_each(|capture| {
                    let count = u32::from_str(capture.get(1).unwrap().as_str()).unwrap();
                    let color = capture.get(2).unwrap().as_str();

                    match color.to_ascii_lowercase().deref() {
                        "red" => red = count,
                        "blue" => blue = count,
                        "green" => green = count,
                        other => panic!("Unknown color: {0}!", other),
                    }
                });

            picks.push(Pick { red, green, blue })
        }

        Ok(Game { id, picks })
    }
}

impl Game {
    fn is_viable(&self, red_limit: u32, green_limit: u32, blue_limit: u32) -> bool {
        self.picks
            .iter()
            .all(|pick| pick.green <= green_limit && pick.red <= red_limit && pick.blue <= blue_limit)
    }

    fn smallest_possible(&self) -> Pick {
        let mut red = 0_u32;
        let mut green = 0_u32;
        let mut blue = 0_u32;

        self.picks.iter().for_each(|pick| {
            red = max(red, pick.red);
            green = max(green, pick.green);
            blue = max(blue, pick.blue);
        });

        Pick { red, green, blue }
    }
}

#[test]
fn test_parse_game() {
    let line = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"#;
    let game = Game::from_str(line).unwrap();

    assert_eq!(game.id, 1);
    assert_eq!(game.picks.len(), 3);

    let picks = &game.picks;
    let a = &picks[0];
    assert_eq!(
        &Pick {
            red: 4,
            blue: 3,
            green: 0,
        },
        a
    );
}


fn games(debug: bool) -> Vec<Game> {
    let file_name = match debug {
        true => "files/day2.example",
        false => "files/day2.puzzle",
    };

    let lines = lines_of_file(file_name).unwrap_or_else(|_| panic!("File not found: {file_name}"));
    lines.into_iter().filter_map(|line| Game::from_str(&line).ok()).collect()
}


pub mod part1 {
    use super::*;

    pub fn run(debug: bool) {
        println!("Day 2 part 1");

        let games = games(debug);

        let valid_sum: u32 = games.iter()
            .filter(|game| game.is_viable(12, 13, 14))
            .map(|game| game.id)
            .sum();

        println!("The sum of the ids of valid games is {valid_sum}");
    }
}

pub mod part2 {
    use super::*;

    pub fn run(debug: bool) {
        println!("Day 2 part 2");

        let games = games(debug);
        let sum_of_power : u32 = games.iter().map(|game| game.smallest_possible().power()).sum();

        println!("The sum of power is {sum_of_power}.");
    }
}
