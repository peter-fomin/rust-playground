pub fn primes_up_to(upper_bound: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = vec![true; upper_bound];
    let mut primes: Vec<usize> = Vec::new();
    for i in 2..=upper_bound {
        if sieve[i - 2] {
            primes.push(i);
            let mut j = 2 * i;
            while j <= upper_bound {
                sieve[j - 2] = false;
                j += i;
            }
        }
    }
    primes
}
