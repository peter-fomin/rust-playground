use std::collections::{HashMap, HashSet};

type ValueMap = HashMap<char, u8>;

pub struct Alphametic<'a> {
    addends: Vec<&'a str>,
    letters: Vec<char>,
    sum: &'a str,
    letter_values: Vec<usize>,
    value_state: [Option<usize>; 10],
    first_letters: HashSet<char>,
}

impl<'a> Alphametic<'a> {
    fn new(addends: Vec<&'a str>, letters: Vec<char>, sum: &'a str) -> Self {
        let length = letters.len();
        let mut alphametic = Self {
            addends,
            letters,
            sum,
            letter_values: vec![0; length],
            value_state: [
                Some(0),
                Some(1),
                Some(2),
                Some(3),
                Some(4),
                Some(5),
                Some(6),
                Some(7),
                Some(8),
                Some(9),
            ],
            first_letters: HashSet::new(),
        };
        alphametic.init_values();
        alphametic
    }

    pub fn solve(&mut self) -> Option<(&Vec<char>, &Vec<usize>)> {
        while !self.is_proper_alphametic() {
            self.increment_values()?;
        }
        Some((&self.letters, &self.letter_values))
    }

    fn init_values(&mut self) {
        for addend in &self.addends {
            self.first_letters.insert(addend.chars().next().unwrap());
        }
        self.first_letters.insert(self.sum.chars().next().unwrap());
        for index in 0..self.letters.len() {
            self.letter_values[index] = self
                .get_next_available_number(self.letter_minimal_number(index))
                .unwrap();
        }
    }

    fn increment_values(&mut self) -> Option<()> {
        let last_index = self.letters.len() - 1;
        self.increment_value_at_index(last_index)?;
        Some(())
    }

    fn increment_value_at_index(&mut self, letter_index: usize) -> Option<()> {
        let value = self.letter_values[letter_index];
        self.value_state[value].replace(value);
        match self.get_next_available_number(value + 1) {
            Some(value) => self.letter_values[letter_index] = value,
            None => {
                if letter_index == 0 {
                    return None;
                }
                self.increment_value_at_index(letter_index - 1)?;
                self.letter_values[letter_index] =
                    self.get_next_available_number(self.letter_minimal_number(letter_index))?;
            }
        }
        Some(())
    }

    fn get_next_available_number(&mut self, mut number: usize) -> Option<usize> {
        loop {
            if number > 9 {
                return None;
            }
            match self.value_state[number].take() {
                Some(number) => return Some(number),
                None => number += 1,
            }
        }
    }

    fn is_proper_alphametic(&self) -> bool {
        self.addends
            .iter()
            .fold(0, |acc, w| acc + self.calculate_word(w))
            == self.calculate_word(self.sum)
    }

    fn calculate_word(&self, word: &str) -> usize {
        word.chars().rev().enumerate().fold(0, |acc, (i, c)| {
            acc + (self.get_letter_value(c)) * (10_usize).pow(i as u32)
        })
    }

    fn get_letter_value(&self, letter: char) -> usize {
        let letter_position = self.letters.iter().position(|&c| c == letter).unwrap();
        self.letter_values[letter_position]
    }

    fn letter_minimal_number(&self, index: usize) -> usize {
        if self.first_letters.contains(&self.letters[index]) {
            1
        } else {
            0
        }
    }
}

pub fn solve(input: &str) -> Option<ValueMap> {
    let mut alphametic = parse_input(input);
    let (letters, values) = alphametic.solve()?;
    Some(format_result(letters, values))
}

fn format_result(letters: &[char], values: &[usize]) -> ValueMap {
    letters
        .iter()
        .zip(values.iter())
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
