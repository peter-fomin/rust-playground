pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = Vec::new();
    let mut m = num;
    while m != 0 {
        digits.push(m % 10);
        m /= 10;
    }
    digits
        .iter()
        .map(|x| x.pow(digits.len() as u32))
        .sum::<u32>()
        == num
}
