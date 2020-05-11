use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiples = HashSet::new();

    for &f in factors {
        let mut num = f;
        while num < limit && num != 0 {
            multiples.insert(num);
            num += f;
        }
    }
    multiples.iter().sum()
}
