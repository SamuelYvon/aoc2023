use std::ops::Deref;
use std::process::exit;
use std::str::FromStr;
use utils::Problem;

pub enum Days {
    Day1,
    Day2,
    Day3,
}

impl FromStr for Days {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().deref() {
            "day1" => Ok(Days::Day1),
            "day2" => Ok(Days::Day2),
            "day3" => Ok(Days::Day3),
            _ => Err(()),
        }
    }
}

impl Days {
    fn run(&self, part: u8, debug: bool) {
        let problem: Box<dyn Problem> = match self {
            // Annoying that I have to box it, and I could do it with unsafe
            // if I wanted to, but meh here.
            Days::Day1 => {
                use day1::DAY_1;
                Box::new(DAY_1)
            }
            Days::Day2 => {
                use day2::DAY_2;
                Box::new(DAY_2)
            }
            Days::Day3 => {
                use day3::DAY_3;
                Box::new(DAY_3)
            }
        };

        if part == 1 {
            problem.get_part1()(debug);
        } else if part == 2 {
            problem.get_part2()(debug);
        } else {
            panic!("Invalid problem part: {0}", part);
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

    let part: u8 =
        u8::from_str(&args[1]).unwrap_or_else(|_| panic!("Invalid part, got {0}", args[1]));
    if part != 1 && part != 2 {
        eprintln!("Part must be one or 2");
        exit(1);
    }

    let type_ = &args[2];
    let type_ = ProblemType::from_str(type_)
        .unwrap_or_else(|| panic!("Not a valid problem type. Got {type_}"));

    let debug = match type_ {
        ProblemType::Example => true,
        ProblemType::Puzzle => false,
    };

    let day = &args[0];
    let day = Days::from_str(day).unwrap_or_else(|_| panic!("Not a valid day entered. Got {day}"));
    day.run(part, debug);
}
