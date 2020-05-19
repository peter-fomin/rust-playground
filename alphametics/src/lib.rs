// This is a solution to alphametic exercise as described in README.md.
// To achieve that we implement an algorithm that solves the alphametic by position:
// from the least significant to the most significant. It works much faster than a simple
// bruteforce algorithm.
// The main idea to deal with a no same digit for different letters rule was to use
// an Option<usize> array, called :digit_store:. When we take a digit for some letter from this
// array we leave None there, and when we no longer need that digit we just return it back.
// Letters are stored in a Vec, called :letters:. Letters in it are sorted by position (least
// significat are first). Corresponding digits to a letter are stored in letter_digits Vec.
// Set called :first_letters: is used to check that those letters cannot be a 0 digit.
// Vec called :position_delimiter: takes an account of which letters correspond to which position.
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
    position_delimeter: Vec<usize>,
}

impl Alphametic {
    fn new(mut operands: Vec<&str>) -> Self {
        // accepts the Vec of operands, the last operand is a sum of previous operands
        let first_letters: BTreeSet<char> =
            operands.iter().map(|w| w.chars().next().unwrap()).collect();

        let sum: Vec<char> = operands.pop().unwrap().chars().rev().collect();
        let addends: Vec<Vec<char>> = operands.iter().map(|w| w.chars().rev().collect()).collect();

        // vector to hold all unique letters per digit position counting from the least significant
        let mut unique_position_letters: Vec<BTreeSet<char>> = Vec::new();
        for (i, _) in sum.iter().enumerate() {
            let mut letter_set: BTreeSet<char> = addends
                .iter()
                .filter_map(|addend| addend.get(i))
                .copied()
                .collect();
            letter_set.insert(sum[i]);
            for previous_set in unique_position_letters.iter().take(i) {
                letter_set = letter_set.difference(previous_set).copied().collect();
            }
            unique_position_letters.push(letter_set);
        }

        let position_delimeter: Vec<usize> = std::iter::once(0)
            .chain(unique_position_letters.iter().scan(0, |s, row| {
                *s += row.len();
                Some(*s)
            }))
            .collect();

        // it is important for letters to be stored in that very order
        let letters: Vec<char> = unique_position_letters
            .into_iter()
            .flat_map(|row| row.into_iter())
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
            position_delimeter,
        }
    }

    pub fn solve(&mut self) -> Option<(&Vec<char>, &Vec<usize>)> {
        if self.try_solve_position(0, 0) {
            Some((&self.letters, &self.letter_digits))
        } else {
            None
        }
    }

    fn try_solve_position(&mut self, position: usize, accumulator: usize) -> bool {
        // Here the main logic resides
        if position == self.sum.len() {
            return true;
        }
        self.init_position(position);
        let mut sum;
        loop {
            sum = self
                .addends
                .iter()
                .filter_map(|w| w.get(position))
                .fold(accumulator, |acc, &c| acc + self.get_letter_digit(c));
            if sum % 10 == self.get_letter_digit(self.sum[position])
                && self.try_solve_position(position + 1, sum / 10)
            {
                // if position is good we check recursively the next one
                return true;
            }
            // if it is not we try incrementing it
            if !self.try_increment_position(position) {
                // if we are unable to increment we return digits back and return to the previous position level
                self.deinit_position(position);
                return false;
            }
        }
    }

    fn try_increment_position(&mut self, position: usize) -> bool {
        let (pos_start, pos_end) = self.position_parameters(position);
        for index in (pos_start..pos_end).rev() {
            // try to increment digits in reverse order
            if self.try_increment_digit_at_index(index) {
                // if successful reinitialize all digits that failed before the current index and return true
                for i in (index + 1)..pos_end {
                    self.set_new_digit_for_letter_at(i);
                }
                return true;
            }
        }
        // if not successfull just return false
        false
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
        (
            self.position_delimeter[position],
            self.position_delimeter[position + 1],
        )
    }

    fn try_increment_digit_at_index(&mut self, letter_index: usize) -> bool {
        let letter_digit = self.letter_digits[letter_index];
        // put the digit back
        self.digit_store[letter_digit] = Some(letter_digit);
        // try to take the next one
        if let Some(digit) = self.take_next_available_digit(letter_digit + 1) {
            self.letter_digits[letter_index] = digit;
            true
        } else {
            false
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
