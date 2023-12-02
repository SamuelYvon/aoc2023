use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::str::FromStr;

pub mod part1 {
    use super::*;

    pub fn calibration_of_line(line: &str) -> u32 {
        let mut int_buff = String::new();

        let chars: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();

        if chars.is_empty() {
            panic!("No characters!")
        }

        int_buff.push(chars[0]);
        int_buff.push(chars[chars.len() - 1]);

        u32::from_str(&int_buff).expect("Should have been a number!")
    }

    #[test]
    fn test_calibration_of_simple_line() {
        assert_eq!(12, calibration_of_line("1abc2"));
    }

    #[allow(dead_code)]
    pub fn run(debug: bool) {
        let filename = if debug {
            "part1.example"
        } else {
            "part1.puzzle"
        };
        let mut file = File::open(Path::new(filename)).expect("Missing part1.puzzle file");
        let mut buff = String::new();

        file.read_to_string(&mut buff)
            .expect("Failed to read the file");

        let total = buff
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(calibration_of_line)
            .sum::<u32>();

        println!("The total is {total}");
    }
}

pub mod part2 {
    use super::*;
    use crate::part1::calibration_of_line;
    use std::collections::HashMap;

    fn replace_words_to_digits(source: &str) -> String {
        let mut word_map = HashMap::new();
        word_map.insert("one", "1");
        word_map.insert("two", "2");
        word_map.insert("three", "3");
        word_map.insert("four", "4");
        word_map.insert("five", "5");
        word_map.insert("six", "6");
        word_map.insert("seven", "7");
        word_map.insert("eight", "8");
        word_map.insert("nine", "9");
        let word_map = word_map;

        let mut replacement_vec: Vec<(usize, usize, &str)> = vec![];

        let mut i = 0;
        // Kinda shot myself in the foot. Can only search for the words and insert the corresponding
        // digit right before, without doing any replacement. Much simpler but oh well.
        while i < source.len() {
            let mut any = false;
            for (word, replacement) in word_map.iter() {
                while i < source.len() && source[i..].starts_with(word) {
                    let x = i + word.len() - 1;
                    replacement_vec.push((i, word.len() - 1, replacement));
                    i = x;
                    any = true;
                }
            }

            if !any {
                i += 1;
            }
        }

        let mut rewritten = String::from(source);

        let n = replacement_vec.len();
        for i in 0..n {
            let len = {
                let (start, len, replacement) = replacement_vec[i];

                let extra = if i == n - 1 { 1 } else { 0 };

                rewritten.replace_range(start..start + len + extra, replacement);
                len
            };

            for (other_start, _, _) in replacement_vec[i + 1..].iter_mut() {
                *other_start -= len - 1;
            }
        }

        #[cfg(debug_assertions)]
        {
            let val = calibration_of_line(&rewritten);
            println!("{source} => {rewritten} ({val})");
        }

        rewritten
    }

    #[allow(dead_code)]
    pub fn run(debug: bool) {
        let filename = if debug {
            "part2.example"
        } else {
            "part2.puzzle"
        };
        let mut file = File::open(Path::new(filename)).expect("Missing part2.puzzle file");
        let mut buff = String::new();

        file.read_to_string(&mut buff)
            .expect("Failed to read the file");

        let total = buff
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(replace_words_to_digits)
            .map(|line| calibration_of_line(&line))
            .sum::<u32>();

        println!("The total is {total}");
    }
}
