use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use utils::Problem;

fn filename(debug: bool) -> &'static str {
    match debug {
        false => "files/day5.puzzle",
        true => "files/day5.example",
    }
}

pub struct Day5();

impl Problem for Day5 {
    fn get_part1(&self) -> fn(bool) -> () {
        part1::solve
    }

    fn get_part2(&self) -> fn(bool) -> () {
        part2::solve
    }
}

pub const DAY_5: Day5 = Day5();

#[derive(Debug)]
struct LargeRange {
    src: u32,
    dest: u32,
    len: u32,
}

impl LargeRange {
    pub fn get(&self, v: u32) -> Option<u32> {
        if v >= self.src && v <= self.src + self.len - 1 {
            let off = v - self.src;
            Some(self.dest + off)
        } else {
            None
        }
    }

    pub fn to_hashmap(&self) -> HashMap<u32, u32> {
        let mut ret = HashMap::new();

        for i in 0..self.len {
            ret.insert(self.src + i, self.dest + i);
        }

        ret
    }
}

#[cfg(test)]
mod test_large_range {
    use crate::LargeRange;

    #[test]
    fn test_basic() {
        let r = LargeRange {
            src: 98,
            dest: 50,
            len: 2,
        };

        assert_eq!(r.get(99), Some(51));
        assert_eq!(r.get(98), Some(50));
        assert!(r.get(100).is_none());
    }
}

fn parse_range(line: &str) -> LargeRange {
    let range_desc = line
        .split(" ")
        .into_iter()
        .filter(|line| line.trim().len() > 0)
        .map(|num_part| u32::from_str(num_part).unwrap())
        .collect::<Vec<_>>();

    assert_eq!(3, range_desc.len());

    let dest = range_desc[0];
    let src = range_desc[1];
    let len = range_desc[2];

    LargeRange { src, dest, len }
}

#[cfg(test)]
mod test_range_parsing {
    use crate::parse_range;
    use std::collections::HashMap;

    #[test]
    fn case1() {
        let line = "50 98 2";
        let parsed = parse_range(line).to_hashmap();

        let mut expected = HashMap::new();
        expected.insert(98_u32, 50_u32);
        expected.insert(99, 51);

        assert_eq!(expected, parsed);
    }

    #[test]
    fn case2() {
        let line = "52 50 48";
        let parsed = parse_range(line).to_hashmap();

        assert_eq!(parsed.len(), 48);
        assert_eq!(parsed.get(&50).unwrap(), &52);
        assert_eq!(parsed.get(&51).unwrap(), &53);
    }
}

#[derive(Debug, Default)]
struct Almanac {
    seeds: Vec<u32>,
    seed_to_soil: Vec<LargeRange>,
    soil_to_fertilizer: Vec<LargeRange>,
    fertilizer_to_water: Vec<LargeRange>,
    water_to_light: Vec<LargeRange>,
    light_to_temperature: Vec<LargeRange>,
    temperature_to_humidity: Vec<LargeRange>,
    humidity_to_location: Vec<LargeRange>,
}

fn get(v: u32, ranges: &Vec<LargeRange>) -> Option<u32> {
    let mut count = 0;
    let mut found = None;

    for r in ranges {
        let candidate = r.get(v);
        if candidate.is_some() {
            count += 1;
            found = candidate;
        }
    }

    assert!(count <= 1);

    found
}

impl Almanac {
    fn destination_of_seed(&self, seed: u32) -> u32 {
        let soil = get(seed, &self.seed_to_soil).unwrap_or(seed);
        let fertilizer = get(soil, &self.soil_to_fertilizer).unwrap_or(soil);
        let water = get(fertilizer, &self.fertilizer_to_water).unwrap_or(fertilizer);
        let light = get(water, &self.water_to_light).unwrap_or(water);
        let temp = get(light, &self.light_to_temperature).unwrap_or(light);
        let hum = get(temp, &self.temperature_to_humidity).unwrap_or(temp);
        get(hum, &self.humidity_to_location).unwrap_or(hum)
    }

    fn destination_of_seeds(&self) -> Vec<u32> {
        let mut destinations = vec![];

        for seed in self.seeds.iter() {
            let seed = *seed;
            let loc = self.destination_of_seed(seed);
            destinations.push(loc);
        }

        destinations
    }
}

fn skip_to_first_non_empty(lines: &Vec<&str>, i: &mut usize) {
    while lines[*i].trim().len() == 0 {
        *i += 1;
    }
}

fn collect_non_empty(lines: &Vec<&str>, i: &mut usize) -> Vec<String> {
    let mut ret = vec![];
    while *i < lines.len() && lines[*i].trim().len() != 0 {
        ret.push(lines[*i].to_string());
        *i += 1;
    }

    ret
}

fn push_all_into(target: &mut Vec<LargeRange>, src: Vec<LargeRange>) {
    for range in src {
        target.push(range)
    }
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut almanac = Almanac::default();
        let lines: Vec<&str> = s.lines().collect();

        let mut i = 0;
        // Read seeds
        let line = &lines[i];
        let seeds_in_str: String = line
            .chars()
            .into_iter()
            .filter(|c| c.is_numeric() || *c == ' ')
            .collect();
        seeds_in_str
            .split(" ")
            .filter(|s| s.trim().len() > 0)
            .for_each(|seed_as_str| {
                almanac.seeds.push(u32::from_str(seed_as_str).unwrap());
            });
        i += 1;

        skip_to_first_non_empty(&lines, &mut i);
        assert_eq!(lines[i], "seed-to-soil map:");
        i += 1;
        let ranges = collect_non_empty(&lines, &mut i)
            .into_iter()
            .map(|sl| parse_range(&sl))
            .collect::<Vec<_>>();
        push_all_into(&mut almanac.seed_to_soil, ranges);
        i += 1;

        assert_eq!(lines[i], "soil-to-fertilizer map:");
        i += 1;
        let ranges = collect_non_empty(&lines, &mut i)
            .into_iter()
            .map(|sl| parse_range(&sl))
            .collect::<Vec<_>>();
        push_all_into(&mut almanac.soil_to_fertilizer, ranges);
        i += 1;

        assert_eq!(lines[i], "fertilizer-to-water map:");
        i += 1;
        let ranges = collect_non_empty(&lines, &mut i)
            .into_iter()
            .map(|sl| parse_range(&sl))
            .collect::<Vec<_>>();
        push_all_into(&mut almanac.fertilizer_to_water, ranges);
        i += 1;

        assert_eq!(lines[i], "water-to-light map:");
        i += 1;
        let ranges = collect_non_empty(&lines, &mut i)
            .into_iter()
            .map(|sl| parse_range(&sl))
            .collect::<Vec<_>>();
        push_all_into(&mut almanac.water_to_light, ranges);
        i += 1;

        assert_eq!(lines[i], "light-to-temperature map:");
        i += 1;
        let ranges = collect_non_empty(&lines, &mut i)
            .into_iter()
            .map(|sl| parse_range(&sl))
            .collect::<Vec<_>>();
        push_all_into(&mut almanac.light_to_temperature, ranges);
        i += 1;

        assert_eq!(lines[i], "temperature-to-humidity map:");
        i += 1;
        let ranges = collect_non_empty(&lines, &mut i)
            .into_iter()
            .map(|sl| parse_range(&sl))
            .collect::<Vec<_>>();
        push_all_into(&mut almanac.temperature_to_humidity, ranges);
        i += 1;

        assert_eq!(lines[i], "humidity-to-location map:");
        i += 1;
        let ranges = collect_non_empty(&lines, &mut i)
            .into_iter()
            .map(|sl| parse_range(&sl))
            .collect::<Vec<_>>();
        push_all_into(&mut almanac.humidity_to_location, ranges);

        Ok(almanac)
    }
}

mod part2 {
    use crate::{filename, Almanac};
    use std::str::FromStr;
    use utils::str_of_file;

    fn run(filename: &str) -> u32 {
        let almanac_as_str = str_of_file(filename).expect("Failed to open the file.");
        let almanac = Almanac::from_str(&almanac_as_str).unwrap();

        let mut lowest = u32::MAX;
        let n = almanac.seeds.len();

        // Make the CPU cry
        for pair_idx in 0..(n / 2) {
            let start = almanac.seeds[pair_idx * 2];
            let len = almanac.seeds[1 + pair_idx * 2];

            for seed in start..(start + len) {
                let loc = almanac.destination_of_seed(seed);
                if loc < lowest {
                    lowest = loc
                }
            }
        }

        lowest
    }

    pub fn solve(debug: bool) {
        let result = run(filename(debug));
        println!("The lowest location number is {result}");
    }
}

mod part1 {
    use crate::{filename, Almanac};
    use std::str::FromStr;
    use utils::str_of_file;

    fn run(filename: &str) -> u32 {
        let almanac_as_str = str_of_file(filename).expect("Failed to open the file.");
        let almanac = Almanac::from_str(&almanac_as_str).unwrap();
        *almanac.destination_of_seeds().iter().min().unwrap()
    }

    pub fn solve(debug: bool) {
        let result = run(filename(debug));
        println!("The lowest location number is {result}");
    }
}
