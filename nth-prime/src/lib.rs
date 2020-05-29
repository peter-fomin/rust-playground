pub fn nth(n: u32) -> u32 {
    // This function uses eratosthene sieve to find nth prime number.
    let n = (n + 1) as usize; // In the task count starts from 0
    let mut primes: Vec<u32> = Vec::with_capacity(n);
    let max_size = if n <= 2 { // Size of eratosthene seieve, for nth prime number should not be more than n * (ln(n) + ln(ln(n)))
        n
    } else {
        let tmp = n as f64;
        (tmp * (tmp.ln() + tmp.ln().ln())) as usize
    };
    let mut eratosthene = vec![true; max_size];
    let mut i: usize = 2;
    while primes.len() < n {
        if eratosthene[i - 2] {
            primes.push(i as u32);
            let mut j = 2 * i;
            while j < max_size {
                eratosthene[j - 2] = false;
                j += i;
            }
        }
        i += 1;
    }
    *primes.last().unwrap()
}
