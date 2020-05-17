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

pub struct Alphametic<'a> {
    addends: Vec<&'a str>,
    letters: Vec<char>,
    sum: &'a str,
    letter_digits: Vec<usize>,
    digit_store: [Option<usize>; 10],
    first_letters: HashSet<char>,
}

impl<'a> Alphametic<'a> {
    fn new(addends: Vec<&'a str>, letters: Vec<char>, sum: &'a str) -> Self {
        let length = letters.len();
        let mut alphametic = Self {
            addends,
            letters,
            sum,
            letter_digits: vec![0; length],
            digit_store: [None; 10],
            first_letters: HashSet::new(),
        };
        alphametic.initialize();
        alphametic
    }

    pub fn solve(&mut self) -> Option<(&Vec<char>, &Vec<usize>)> {
        while !self.is_proper_alphametic() {
            self.increment_digits()?;
        }
        Some((&self.letters, &self.letter_digits))
    }

    fn initialize(&mut self) {
        // populate value_state
        (0..10).for_each(|i| self.digit_store[i] = Some(i));
        // populate first_letters
        for addend in &self.addends {
            self.first_letters.insert(addend.chars().next().unwrap());
        }
        self.first_letters.insert(self.sum.chars().next().unwrap());
        // initialize letter_values
        for index in 0..self.letters.len() {
            self.letter_digits[index] = self
                .take_next_available_digit(self.letter_starting_digit(index))
                .unwrap();
        }
    }

    fn increment_digits(&mut self) -> Option<()> {
        // increment letter_values starting from the last index
        // if we incremented through all possible value we return None
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
                self.letter_digits[letter_index] =
                    self.take_next_available_digit(self.letter_starting_digit(letter_index))?;
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
        // calculate left and right parts of the equation adn compare them
        self.addends
            .iter()
            .fold(0, |acc, w| acc + self.calculate_word(w))
            == self.calculate_word(self.sum)
    }

    fn calculate_word(&self, word: &str) -> usize {
        word.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            acc + (self.get_letter_digit(c)) * (10_usize).pow(i as u32)
        })
    }

    fn get_letter_digit(&self, letter: char) -> usize {
        let letter_position = self.letters.iter().position(|&c| c == letter).unwrap();
        self.letter_digits[letter_position]
    }

    fn letter_starting_digit(&self, index: usize) -> usize {
        match self.first_letters.contains(&self.letters[index]) {
            true => 1,
            false => 0,
        }
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
    let mut addends: Vec<&str> = input
        .split(|c: char| !c.is_alphabetic())
        .filter(|w| !w.is_empty())
        .collect();
    let sum = addends.pop().unwrap();
    let letters: HashSet<char> = input.chars().filter(|c| c.is_alphabetic()).collect();
    let mut letters: Vec<char> = letters.into_iter().collect();
    letters.sort();
    Alphametic::new(addends, letters, sum)
}
