use std::fmt::{Debug, Formatter, Write};
use std::str::FromStr;
use utils::{str_of_file, Problem};

pub struct Day3();

impl Problem for Day3 {
    fn get_part1(&self) -> fn(bool) -> () {
        part1::run
    }

    fn get_part2(&self) -> fn(bool) -> () {
        part2::run
    }
}

pub const DAY_3: Day3 = Day3();

fn filename(debug: bool) -> &'static str {
    match debug {
        true => "files/day3.example",
        false => "files/day3.puzzle",
    }
}

struct Matrix<T: Default + Copy + Clone + Debug> {
    dims: (usize, usize),
    vals: Vec<T>,
}

impl FromStr for Matrix<char> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();
        let n = lines.len();
        let m = lines.iter().next().unwrap().len();

        let mut mat = Self::new(n, m);

        for (row, line) in lines.iter().enumerate() {
            for (col, char) in line.chars().enumerate() {
                mat.set(char, row, col)
            }
        }

        Ok(mat)
    }
}

impl<T: Default + Copy + Clone + Debug> Matrix<T> {
    pub fn new(rows: usize, cols: usize) -> Self {
        let vals = vec![T::default(); rows * cols];
        Matrix {
            dims: (rows, cols),
            vals,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> T {
        let (_, cols) = self.dims;
        self.vals[row * cols + col]
    }

    pub fn set(&mut self, val: T, row: usize, col: usize) {
        let (_, cols) = self.dims;
        self.vals[row * cols + col] = val
    }

    pub fn dims(&self) -> (usize, usize) {
        self.dims
    }
}

impl<T: Default + Copy + Clone + Debug> Debug for Matrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (rows, cols) = self.dims;
        let mut buff = String::new();

        f.write_char('\n')?;
        for row in 0..rows {
            for col in 0..cols {
                buff.write_str(&format!("{:#?} ", self.get(row, col)))?;
            }
            buff.write_char('\n')?;
        }

        f.write_str(&buff)
    }
}

impl<T: Clone + Copy + Default + Debug> Clone for Matrix<T> {
    fn clone(&self) -> Self {
        Matrix {
            dims: self.dims,
            vals: self.vals.clone(),
        }
    }
}

struct NumberPair {
    number1: u32,
    number2: u32,
}

impl Matrix<char> {
    pub fn mask(&mut self, replacement: char, other: &Matrix<bool>) {
        let (rows, cols) = other.dims;

        for row in 0..rows {
            for col in 0..cols {
                if !other.get(row, col) {
                    self.set(replacement, row, col);
                }
            }
        }
    }

    pub fn extract_pairs(&self, m: &Matrix<bool>) -> Vec<NumberPair> {
        let mut copy = self.clone();
        let mut vec = vec![];
        let (rows, cols) = self.dims;

        for row in 0..rows {
            for col in 0..cols {
                let sym = copy.get(row, col);
                if sym == '*' {

                    let mut num1 : Option<u32> = None;
                    let mut num2 : Option<u32> = None;

                    for i in -1..=1 {
                        for j in -1..=1 {
                            if i == 0 && j == 0 {
                                continue;
                            }

                            let row = row as isize + i;
                            let col = col as isize + j;

                            if row < 0 || row as usize > rows || col < 0 || col as usize > rows {
                                continue;
                            }

                            let row = row as usize;
                            let col = col as usize;

                            let mut s = col;
                            let mut e = col;

                            // Go backwards
                            while s != 0 && copy.get(row, s - 1).is_numeric() {
                                s -= 1;
                            }

                            // Go forward
                            while e < cols && copy.get(row, e + 1).is_numeric() {
                                e += 1;
                            }

                            // TODO: erase afterwards
                            let range = row * cols + s..=row * cols + e;
                            let slice = &copy.vals[range.clone()];

                            let s : String = slice.iter().collect();
                            let number = match u32::from_str(&s) {
                                Ok(num) => num,
                                Err(_) => continue
                            };
                            copy.vals[range.clone()].iter_mut().for_each(|c| *c = '.');

                            if num1.is_none() {
                                num1 = Some(number);
                            } else {
                                num2 = Some(number);
                            }
                        }
                    }

                    if num1.is_some() && num2.is_some() {
                        vec.push(
                            NumberPair {
                                number1: num1.unwrap(),
                                number2: num2.unwrap()
                            }
                        )

                    }
                }
            }
        }

        vec
    }

    pub fn extract_numbers(&self) -> Vec<u32> {
        let mut vec = vec![];

        let (rows, cols) = self.dims;

        let mut buff = String::new();
        for row in 0..rows {
            for col in 0..cols {
                let c = self.get(row, col);
                if c.is_numeric() {
                    buff.push(c);
                } else if buff.len() > 0 {
                    vec.push(u32::from_str(&buff).unwrap());
                    buff.clear();
                }
            }

            if buff.len() > 0 {
                vec.push(u32::from_str(&buff).unwrap());
                buff.clear();
            }
        }

        vec
    }
}

#[derive(Copy, Clone)]
struct NumPart {
    row: usize,
    col: usize,
}

impl Matrix<bool> {
    fn any_adjacent_ok(row: usize, col: usize, matrix: &Matrix<char>) -> bool {
        let (rows, cols) = matrix.dims;

        let mut any_ok = false;

        for i in -1..=1 {
            for j in -1..=1 {
                let row = (row as isize) + i;
                let col = (col as isize) + j;

                let row_oob = row < 0 || (row as usize) >= rows;
                let col_oob = col < 0 || (col as usize) >= cols;

                if row_oob || col_oob || (i == 0 && j == 0) {
                    continue;
                }

                let sym = matrix.get(row as usize, col as usize);

                any_ok |= "()+-#*&$@=/?!%".contains(sym);

                if any_ok {
                    break;
                }
            }
        }

        any_ok
    }

    pub fn or(&mut self, other: &Matrix<bool>) {
        let (rows, cols) = self.dims;
        for row in 0..rows {
            for col in 0..cols {
                let v = self.get(row, col);
                let o = other.get(row, col);
                self.set(v || o, row, col);
            }
        }
    }

    pub fn find_gears(
        &mut self,
        number_matrix: &Matrix<bool>,
        sym_matrix: &Matrix<char>,
    ) {
        let (rows, cols) = self.dims;

        let mut pairs: Vec<(NumPart, NumPart)> = vec![];
        let mut temp = vec![];

        for row in 0..rows {
            for col in 0..cols {
                // Is it a gear?
                let sym = sym_matrix.get(row, col);
                if sym != '*' {
                    continue;
                }

                // Check if two numbers next to it
                let mut count = 0_u32;

                for i in -1..=1 {
                    for j in -1..=1 {
                        let col = col as isize + i;
                        let row = row as isize + j;

                        // check oob
                        if col < 0 || row < 0 || col as usize >= cols || row as usize >= rows {
                            continue;
                        }

                        let row = row as usize;
                        let col = col as usize;
                        let has_num = number_matrix.get(row, col);

                        if has_num {
                            temp.push(NumPart { row, col });
                        }

                        count += has_num as u32;
                        if temp.len() == 2 {
                            pairs.push((temp[0], temp[1]));
                            temp.clear();
                            break;
                        }
                    }
                }

                self.set(count >= 2, row, col);
            }
        }
    }

    pub fn keep_gear_adjacent(&mut self, gear_matrix: &Matrix<bool>) {
        let (rows, cols) = self.dims;
        let old = self.clone();

        for row in 0..rows {
            for col in 0..cols {
                let mut any = false;

                for i in -1..=1 {
                    for j in -1..=1 {
                        let row = (row as isize) + i;
                        let col = (col as isize) + j;

                        let row_oob = row < 0 || (row as usize) >= rows;
                        let col_oob = col < 0 || (col as usize) >= cols;

                        if row_oob || col_oob || (i == 0 && j == 0) {
                            continue;
                        }

                        let row = row as usize;
                        let col = col as usize;

                        any |= gear_matrix.get(row, col);
                    }
                }

                self.set(any && old.get(row, col), row, col);
            }
        }

        // Sideway scan left & right to complete the number
        for row in 0..rows {
            for col in 0..cols {
                let val = self.get(row, col);

                if !val {
                    continue;
                }

                let i = col;
                for j in 0..cols {
                    // Check oob on the right
                    if i + j >= cols {
                        break;
                    }

                    let candidate = old.get(row, i + j);
                    if candidate {
                        self.set(true, row, i + j);
                    } else {
                        break;
                    }
                }

                let i = col as isize;
                for j in 0..(cols as isize) {
                    // Check oob on the right
                    if i - j < 0 {
                        break;
                    }

                    let candidate = old.get(row, (i - j) as usize);
                    if candidate {
                        self.set(true, row, (i - j) as usize);
                    } else {
                        break;
                    }
                }
            }
        }
    }

    pub fn sum(&self) -> u32 {
        self.vals.iter().map(|v| *v as u32).sum()
    }

    pub fn check_numbers(&mut self, text_matrix: &Matrix<char>) {
        let (b_rows, b_cols) = self.dims;
        let (c_rows, c_cols) = text_matrix.dims();

        // Make sure it fits
        assert_eq!(b_cols, c_cols);
        assert_eq!(b_rows, c_rows);

        for row in 0..b_rows {
            for col in 0..c_cols {
                if text_matrix.get(row, col).is_numeric() {
                    self.set(Self::any_adjacent_ok(row, col, text_matrix), row, col)
                }
            }
        }

        for row in 0..b_rows {
            for col in 0..b_cols {
                let val = self.get(row, col);

                // Sideway scan left & right to complete the number
                if !val {
                    continue;
                }

                let i = col;
                for j in 0..b_rows {
                    // Check oob on the right
                    if i + j >= b_cols {
                        break;
                    }

                    let candidate = text_matrix.get(row, i + j);
                    if candidate.is_numeric() {
                        self.set(true, row, i + j);
                    } else {
                        break;
                    }
                }

                let i = col as isize;
                for j in 0..(b_rows as isize) {
                    // Check oob on the right
                    if i - j < 0 {
                        break;
                    }

                    let candidate = text_matrix.get(row, (i - j) as usize);
                    if candidate.is_numeric() {
                        self.set(true, row, (i - j) as usize);
                    } else {
                        break;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::Matrix;

    #[test]
    pub fn test_access() {
        let mut matrix = Matrix::new(10, 10);
        assert_eq!('\0', matrix.get(0, 0));
        matrix.set('v', 9, 0);
        assert_eq!('v', matrix.get(9, 0));
        dbg!(matrix.vals);
    }
}

#[cfg(test)]
mod part1_tests {
    use crate::part1::solve;

    #[test]
    fn case1() {
        let s = "........
.24..4..
......*.";

        assert_eq!(4, solve(s));
    }

    #[test]
    fn case2() {
        let s = r#"........
.24$-4..
......*."#;

        assert_eq!(4 + 24, solve(s));
    }

    #[test]
    fn case3() {
        let s = r#"11....11
..$..$..
11....11"#;
        assert_eq!(4 * 11, solve(s));
    }

    #[test]
    fn case4() {
        let s = r#"$......$
.1....1.
.1....1.
$......$"#;
        assert_eq!(4, solve(s));
    }

    #[test]
    fn case5() {
        let s = r#"$......$
.11..11.
.11..11.
$......$"#;
        assert_eq!(4 * 11, solve(s));
    }

    #[test]
    fn case6() {
        let s = r#"$11
...
11$
..."#;
        assert_eq!(22, solve(s));
    }

    #[test]
    fn case7() {
        let s = r#"$..
.11
.11
$..
..$
11.
11.
..$"#;
        assert_eq!(4 * 11, solve(s));
    }

    #[test]
    fn case8() {
        let s = r#"11.$."#;
        assert_eq!(0, solve(s));
    }
}

mod part2 {
    use super::*;

    pub fn solve(s: &str) -> u32 {
        let mut sum = 0_u32;
        let mut text_matrix = Matrix::from_str(s).unwrap();
        let (rows, cols) = text_matrix.dims();

        let mut boolean_matrix: Matrix<bool> = Matrix::new(rows, cols);
        boolean_matrix.check_numbers(&text_matrix);

        let mut gear_matrix: Matrix<bool> = Matrix::new(rows, cols);
        let number_pairs = gear_matrix.find_gears(&boolean_matrix, &text_matrix);

        println!("Found {0} gears.", gear_matrix.sum());

        boolean_matrix.keep_gear_adjacent(&gear_matrix);

        // Remove the non-number stuff
        boolean_matrix.or(&gear_matrix);
        text_matrix.mask('.', &boolean_matrix);

        dbg!(&boolean_matrix);
        dbg!(&text_matrix);

        let pairs = text_matrix.extract_pairs(&boolean_matrix);

        sum = pairs.iter().map(|pair| pair.number1 * pair.number2).sum();

        sum
    }

    pub fn run(debug: bool) {
        let s = &str_of_file(filename(debug)).unwrap();
        let result = solve(s);
        println!("Gear ratio is: {result}")
    }
}

mod part1 {
    use super::*;

    pub fn solve(s: &str) -> u32 {
        let mut text_matrix = Matrix::from_str(s).unwrap();
        let (rows, cols) = text_matrix.dims();

        let mut boolean_matrix: Matrix<bool> = Matrix::new(rows, cols);
        boolean_matrix.check_numbers(&text_matrix);
        text_matrix.mask('.', &boolean_matrix);

        let numbers = text_matrix.extract_numbers();
        numbers.into_iter().sum::<u32>()
    }

    pub fn run(debug: bool) {
        let s = &str_of_file(filename(debug)).unwrap();
        let result = solve(s);
        println!("Sum is: {result}")
    }
}
