pub fn raindrops(n: u32) -> String {
    let mut raindrops_sound = String::new();
    if n % 3 == 0 {
        raindrops_sound.push_str("Pling");
    }
    if n % 5 == 0 {
        raindrops_sound.push_str("Plang");
    }
    if n % 7 == 0 {
        raindrops_sound.push_str("Plong");
    }
    if raindrops_sound.is_empty() {
        raindrops_sound = n.to_string()
    }
    raindrops_sound
}
