use std::arch::x86_64::_bextr2_u64;
use std::str::FromStr;
use utils::{filename, str_of_file, Problem};

pub struct Day6();

impl Problem for Day6 {
    fn get_part1(&self) -> fn(bool) -> () {
        part1::run
    }

    fn get_part2(&self) -> fn(bool) -> () {
        part2::run
    }
}

pub const DAY_6: Day6 = Day6();

pub struct Races {
    times: Vec<u64>,
    dists: Vec<u64>,
}

impl Races {
    pub fn into_super_race(self) -> Self {
        let time = self.times.iter().map(|x| x.to_string()).reduce(|a, b| format!("{a}{b}")).unwrap();
        let dist = self.dists.iter().map(|x| x.to_string()).reduce(|a, b| format!("{a}{b}")).unwrap();

        let i =5;
        Races {
            times : vec![u64::from_str(&time).unwrap()],
            dists : vec![u64::from_str(&dist).unwrap()]
        }
    }
}

fn num_beats(time: u64, dist: u64) -> u64 {
    let mut can_beat = 0;
    for held in 1..time {
        let speed = held;
        let time_left = time - held;
        let traveled = time_left * speed;

        can_beat += (traveled > dist) as u64
    }

    can_beat
}

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();
        assert_eq!(2, lines.len());

        let times = lines[0]
            .split_whitespace()
            .skip(1)
            .map(|num| u64::from_str(num).unwrap())
            .collect::<Vec<_>>();
        let dists = lines[1]
            .split_whitespace()
            .skip(1)
            .map(|num| u64::from_str(num).unwrap())
            .collect::<Vec<_>>();

        Ok(Races { times, dists })
    }
}

mod part1 {
    use super::*;

    fn solve(s: &str) -> u64 {
        let races = Races::from_str(s).unwrap();

        races
            .times
            .iter()
            .zip(races.dists)
            .map(|(time, dist)| num_beats(*time, dist))
            .reduce(|a, b| a * b).unwrap()
    }

    pub fn run(debug: bool) {
        let filename = filename(6, debug);
        println!(
            "Result of multiplication: {0}",
            solve(&str_of_file(&filename).unwrap())
        );
    }
}

mod part2 {
    use super::*;
    fn solve(s: &str) -> u64 {
        let races = Races::from_str(s).unwrap();
        let races = races.into_super_race();

        races
            .times
            .iter()
            .zip(races.dists)
            .map(|(time, dist)| num_beats(*time, dist))
            .reduce(|a, b| a * b).unwrap()
    }

    pub fn run(debug: bool) {
        let filename = filename(6, debug);
        println!(
            "Result of super-race: {0}",
            solve(&str_of_file(&filename).unwrap())
        );
    }
}
