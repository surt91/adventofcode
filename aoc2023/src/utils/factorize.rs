/// Finds all primefactors of the given number, returned as a vector.
///
/// # Examples
///
/// ```
/// use aoc2023::utils::factorize::factorize;
///
/// assert_eq!(vec![7, 191], factorize(1337));
/// ```
pub fn factorize(mut number: usize) -> Vec<usize> {
    let primes = super::eratosthenes::sieve_of_eratosthenes((number as f64).sqrt() as usize + 1);
    let mut ret = Vec::new();

    'outer: while number > 1 {
        for prime in &primes {
            if number % prime == 0 {
                number /= prime;
                ret.push(*prime);
                continue 'outer;
            }
        }
        // the remainder will be prime
        ret.push(number);
        break;
    }

    ret
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factors() {
        assert!(factorize(12) == vec![2, 2, 3]);
        assert!(factorize(13) == vec![13]);
        assert!(factorize(14) == vec![2, 7]);
    }
}