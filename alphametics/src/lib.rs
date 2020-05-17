// This is a solution to alphametic exercise as described in README.md.
// To achieve that we just bruteforce through all possible variants. It could have been done
// with a use of external crate to generate possible permutations, and I think it would have
// been faster, but I tried to implement that myself.
// The main idea is to use array of Option<usize> called digit_store. Since two letters can not
// represent the same digit for a digit to be put into letter_digits Vec it has to be taken from
// digit_store array and also be returned later.
// Letters are stored in letters Vec, their corresponding digits are stored in letter_digits Vec.
// first_letters contains letters that cannot be zero due to being the first letter in the word.
use std::collections::{HashMap, HashSet};

type DigitMap = HashMap<char, u8>;

pub struct Alphametic {
    // addends and letters are stored in reverse order for direct letter access
    addends: Vec<Vec<char>>,
    letters: Vec<char>,
    sum: Vec<char>,
    letter_digits: Vec<usize>,
    digit_store: [Option<usize>; 10],
    first_letters: HashSet<char>,
}

impl Alphametic {
    fn new(mut operands: Vec<&str>) -> Self {
        // accepts the Vec of operands, the last one should be the sum of previous addends
        let letters: HashSet<char> = operands.iter().flat_map(|w| w.chars()).collect();
        let mut letters: Vec<char> = letters.into_iter().collect();
        letters.sort();

        let letter_digits = vec![0; letters.len()];

        let first_letters: HashSet<char> =
            operands.iter().map(|w| w.chars().next().unwrap()).collect();

        let sum = operands.pop().unwrap().chars().rev().collect();
        let addends = operands.iter().map(|w| w.chars().rev().collect()).collect();

        let mut digit_store = [None; 10];
        (0..10).for_each(|i| digit_store[i] = Some(i));

        let mut alphametic = Self {
            addends,
            letters,
            sum,
            letter_digits,
            digit_store,
            first_letters,
        };

        for index in 0..alphametic.letters.len() {
            alphametic.set_new_digit_for_letter_at(index);
        }
        alphametic
    }

    pub fn solve(&mut self) -> Option<(&Vec<char>, &Vec<usize>)> {
        while !self.is_proper_alphametic() {
            self.increment_digits()?;
        }
        Some((&self.letters, &self.letter_digits))
    }

    fn increment_digits(&mut self) -> Option<()> {
        // increment letter_values starting from the last index
        // if we incremented through all possible values we return None
        let last_index = self.letters.len() - 1;
        self.increment_digit_at_index(last_index)?;
        Some(())
    }

    fn increment_digit_at_index(&mut self, letter_index: usize) -> Option<()> {
        let letter_digit = self.letter_digits[letter_index];
        // put the digit back
        self.digit_store[letter_digit] = Some(letter_digit);
        // try to take the next one
        match self.take_next_available_digit(letter_digit + 1) {
            Some(digit) => self.letter_digits[letter_index] = digit,
            None => {
                if letter_index == 0 {
                    // no more letters
                    return None;
                }
                self.increment_digit_at_index(letter_index - 1)?;
                self.set_new_digit_for_letter_at(letter_index);
            }
        }
        Some(())
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

    fn is_proper_alphametic(&self) -> bool {
        // for every digit calculate a sum and compare it with the corresponding digit in the self.sum
        let mut sum = 0;
        for (i, &s) in self.sum.iter().enumerate() {
            sum = self
                .addends
                .iter()
                .filter_map(|w| w.get(i))
                .fold(sum, |acc, &c| acc + self.get_letter_digit(c));
            let digit = sum % 10;
            if digit != self.get_letter_digit(s) {
                return false;
            }
            sum /= 10;
        }
        true
    }

    fn get_letter_digit(&self, letter: char) -> usize {
        let letter_position = self.letters.binary_search(&letter).unwrap();
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
