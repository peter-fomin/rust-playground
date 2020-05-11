fn verse(word_1: &str, word_2: &str) -> String {
    format!("For want of a {0} the {1} was lost.", word_1, word_2)
}

pub fn build_proverb(list: &[&str]) -> String {
    if !list.is_empty() {
        let mut proverb: Vec<String> = (1..list.len())
            .map(|i| verse(list[i - 1], list[i]))
            .collect();
        proverb.push(format!("And all for the want of a {}.", list[0]));
        proverb.join("\n")
    } else {
        String::new()
    }
}
