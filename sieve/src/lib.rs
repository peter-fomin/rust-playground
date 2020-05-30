pub fn primes_up_to(upper_bound: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = vec![true; upper_bound + 1];
    let mut primes: Vec<usize> = Vec::new();
    for i in 2..=upper_bound {
        if sieve[i] {
            primes.push(i);
            for j in ((i * i)..=upper_bound).step_by(i) {
                sieve[j] = false;
            }
        }
    }
    primes
}
