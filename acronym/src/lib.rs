pub fn abbreviate(phrase: &str) -> String {
    phrase
        .split(|c: char| !c.is_alphabetic() && c != '\'')
        .filter(|word| !word.is_empty())
        .flat_map(|word| {
            word.chars().next().unwrap().to_uppercase().chain(
                word.chars()
                    .skip_while(|c| c.is_uppercase())
                    .filter(|c| c.is_uppercase()),
            )
        })
        .collect()
}
