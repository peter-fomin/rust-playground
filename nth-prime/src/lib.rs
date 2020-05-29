pub fn nth(n: usize) -> usize {
    // This function uses eratosthene sieve to find nth prime number.
    let n = n + 1; // In the task count starts from 0
    let max_size = if n <= 2 { // Size of eratosthene seieve, for nth prime number should not be more than n * (ln(n) + ln(ln(n)))
        n
    } else {
        let tmp = n as f64;
        (tmp * (tmp.ln() + tmp.ln().ln())) as usize
    };
    let mut sieve = vec![true; max_size];
    let mut i = 2;
    let mut count = 0;
    while count < n {
        if sieve[i - 2] {
            count += 1;
            let mut j = 2 * i;
            while j < max_size {
                sieve[j - 2] = false;
                j += i;
            }
        }
        i += 1;
    }
    i - 1
}
