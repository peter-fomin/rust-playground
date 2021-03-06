fn bottles(n: u32) -> String {
    match n {
        1 => String::from("1 bottle"),
        _ => format!("{} bottles", n),
    }
}

pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n"),
        1 => String::from("1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n"),
        _ => format!("{0} of beer on the wall, {0} of beer.\nTake one down and pass it around, {1} of beer on the wall.\n", bottles(n), bottles(n - 1)),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
