// This is a solution to alphametic exercise as described in README.md.
// To achieve that we just bruteforce through all possible variants. It could have been done
// with a use of external crate to generate possible permutations, and I think it would have
// been faster, but I tried to implement that myself.
// The main idea is to use array of Option<usize> called digit_store. Since two letters can not
// represent the same digit for a digit to be put into letter_digits Vec it has to be taken from
// digit_store array and also be returned later.
// Letters are stored in letters Vec, their corresponding digits are stored in letter_digits Vec.
// first_letters contains letters that cannot be zero due to being the first letter in the word.
use std::collections::{BTreeSet, HashMap};

type DigitMap = HashMap<char, u8>;

pub struct Alphametic {
    // addends and sum are stored in reverse order for direct letter access
    addends: Vec<Vec<char>>,
    letters: Vec<char>,
    sum: Vec<char>,
    letter_digits: Vec<usize>,
    digit_store: [Option<usize>; 10],
    first_letters: BTreeSet<char>,
    position_end: Vec<usize>,
}

impl Alphametic {
    fn new(mut operands: Vec<&str>) -> Self {
        // accepts the Vec of operands, the last one should be the sum of previous addends
        let first_letters: BTreeSet<char> =
            operands.iter().map(|w| w.chars().next().unwrap()).collect();

        let sum: Vec<char> = operands.pop().unwrap().chars().rev().collect();
        let addends: Vec<Vec<char>> = operands.iter().map(|w| w.chars().rev().collect()).collect();

        // vector to hold all unique letters per digit position counting from the least significant
        let mut unique_letters: Vec<BTreeSet<char>> = Vec::new();
        for (i, _) in sum.iter().enumerate() {
            let mut letter_set: BTreeSet<char> = addends
                .iter()
                .filter_map(|addend| addend.get(i))
                .copied()
                .collect();
            letter_set.insert(sum[i]);
            for previous_set in unique_letters.iter().take(i) {
                letter_set = letter_set.difference(previous_set).copied().collect();
            }
            unique_letters.push(letter_set);
        }

        let position_end: Vec<usize> = unique_letters.iter().scan(0, |s, row| {*s += row.len(); Some(*s)}).collect();

        let letters: Vec<char> = unique_letters
            .into_iter()
            .flat_map(|pos| pos.into_iter())
            .collect();

        let letter_digits = vec![0; letters.len()];

        let mut digit_store = [None; 10];
        (0..10).for_each(|i| digit_store[i] = Some(i));

        Self {
            addends,
            letters,
            sum,
            letter_digits,
            digit_store,
            first_letters,
            position_end,
        }
    }

    pub fn solve(&mut self) -> Option<(&Vec<char>, &Vec<usize>)> {
        self.solve_position(0, 0)?;
        Some((&self.letters, &self.letter_digits))
    }

    fn solve_position(&mut self, position: usize, accumulator: usize) -> Option<()> {
        if position == self.sum.len() {
            return Some(());
        }
        // let position_length = self.letters_at_position[position];
        self.init_position(position);
        let mut sum;
        loop {
            sum = self.addends.iter().filter_map(|w| w.get(position)).fold(accumulator, |acc, &c| acc + self.get_letter_digit(c));
            if sum % 10 == self.get_letter_digit(self.sum[position]) {
                if let Some(_) = self.solve_position(position + 1, sum / 10) {
                    return Some(());
                }
            }
            if let None = self.increment_position(position) {
                self.deinit_position(position);
                return None;
            }
        }
    }

    fn increment_position(&mut self, position: usize) -> Option<()> {
        let (pos_start, pos_end) = self.position_parameters(position);
        for index in (pos_start..pos_end).rev() {
            if let Some(_) = self.increment_digit_at_index(index) {
                for i in (index + 1)..pos_end {
                    self.set_new_digit_for_letter_at(i);
                }
                return Some(());
            }
        }
        None
    }

    fn init_position(&mut self, position: usize) {
        let (pos_start, pos_end) = self.position_parameters(position);
        for letter_index in pos_start..pos_end {
            self.set_new_digit_for_letter_at(letter_index);
        }
    }

    fn deinit_position(&mut self, position: usize) {
        let (pos_start, pos_end) = self.position_parameters(position);
        for letter_index in pos_start..pos_end {
            let value = self.letter_digits[letter_index];
            self.digit_store[value] = Some(value);
        }
    }

    fn position_parameters(&self, position: usize) -> (usize, usize) {
        let position_start = if position == 0 {
            0
        } else {
            self.position_end[position - 1]
        };
        let position_end = self.position_end[position];
        (position_start, position_end)
    }

    fn increment_digit_at_index(&mut self, letter_index: usize) -> Option<()> {
        let letter_digit = self.letter_digits[letter_index];
        // put the digit back
        self.digit_store[letter_digit] = Some(letter_digit);
        // try to take the next one
        if let Some(digit) = self.take_next_available_digit(letter_digit + 1) {
            self.letter_digits[letter_index] = digit;
            Some(())
        } else {
            None
        }
    }

    fn take_next_available_digit(&mut self, mut digit: usize) -> Option<usize> {
        loop {
            if digit > 9 {
                return None;
            }
            match self.digit_store[digit].take() {
                Some(digit) => return Some(digit),
                None => digit += 1,
            }
        }
    }

    fn get_letter_digit(&self, letter: char) -> usize {
        let letter_position = self.letters.iter().position(|&c| c == letter).unwrap();
        self.letter_digits[letter_position]
    }

    fn set_new_digit_for_letter_at(&mut self, index: usize) {
        let starting_digit = if self.first_letters.contains(&self.letters[index]) {
            1
        } else {
            0
        };
        self.letter_digits[index] = self.take_next_available_digit(starting_digit).unwrap();
    }
}

pub fn solve(input: &str) -> Option<DigitMap> {
    let mut alphametic = parse_input(input);
    let (letters, digits) = alphametic.solve()?;
    Some(format_result(letters, digits))
}

fn format_result(letters: &[char], digits: &[usize]) -> DigitMap {
    letters
        .iter()
        .zip(digits.iter())
        .map(|(&l, &v)| (l, v as u8))
        .collect()
}

fn parse_input(input: &str) -> Alphametic {
    let operands: Vec<&str> = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .collect();
    Alphametic::new(operands)
}
