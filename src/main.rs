use std::ops::Deref;
use std::process::exit;
use std::str::FromStr;

pub enum Days {
    Day1,
    Day2,
}

impl FromStr for Days {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().deref() {
            "day1" => Ok(Days::Day1),
            "day2" => Ok(Days::Day2),
            _ => Err(()),
        }
    }
}

enum ProblemType {
    Example,
    Puzzle,
}

impl ProblemType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().deref() {
            "example" => Some(Self::Example),
            "puzzle" => Some(Self::Puzzle),
            _ => None,
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.len() != 3 {
        eprintln!("Usage: ./aoc2023 DAY PART TYPE");
        exit(1);
    }

    let day = &args[0];
    let part: u8 = u8::from_str(&args[1]).unwrap_or_else(|_| panic!("Invalid part, got {0}", args[1]));

    if part != 1 && part != 2 {
        eprintln!("Part must be one or 2");
        exit(1);
    }

    let type_ = &args[2];

    let day = Days::from_str(day).unwrap_or_else(|_| panic!("Not a valid day entered. Got {day}"));
    let type_ =
        ProblemType::from_str(type_).unwrap_or_else(|| panic!("Not a valid problem type. Got {type_}"));

    let debug = match type_ {
        ProblemType::Example => true,
        ProblemType::Puzzle => false,
    };

    match day {
        Days::Day1 => {
            if part == 1 {
                use day1::part1::run;
                run(debug);
            } else {
                use day1::part2::run;
                run(debug);
            }
        }
        Days::Day2 => {
            if part == 1 {
                use day2::part1::run;
                run(debug);
            } else {
                use day2::part2::run;
                run(debug);
            }
        }
    }
}
