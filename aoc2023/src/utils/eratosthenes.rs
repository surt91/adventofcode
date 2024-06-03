use std::cmp;

/// Finds all primes smaller or equal than limit and returns them in a vector.
///
/// # Examples
///
/// ```
/// let primes = aoc2023::utils::eratosthenes::sieve_of_eratosthenes(20);
///
/// assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19], primes);
/// ```
pub fn sieve_of_eratosthenes(limit: usize) -> Vec<usize> {
    let mut sieve: Vec<bool> = vec![true; limit+1];
    let mut out: Vec<usize> = Vec::new();
    let ub: usize = (limit as f64).sqrt() as usize;

    // collect seed primes
    let seed_primes = if ub <= 20 {
        vec![2, 3, 5, 7, 11, 13, 17, 19]
    } else {
        sieve_of_eratosthenes(ub)
    };

    // start the sieving
    sieve[0] = false;
    sieve[1] = false;
    for i in seed_primes {
        let mut j = i * 2;
        while j < limit + 1 {
            sieve[j] = false;
            j += i;
        }
    }

    // read the results
    for i in 2..=limit {
        if sieve[i] {
            out.push(i);
        }
    }

    out
}

pub fn segmented_sieve_of_eratosthenes(limit: usize, segment_size_opt: Option<usize>) -> Vec<usize> {
    let ub: usize = (limit as f64).sqrt() as usize;
    let segment_size = segment_size_opt.unwrap_or(28_000);

    let mut sieve: Vec<bool> = vec![true; segment_size];
    sieve[0] = false;
    sieve[1] = false;
    let mut out: Vec<usize> = Vec::new();
    out.push(2);

    // collect seed primes
    let seed_primes = if ub <= 20 {
        vec![3, 5, 7, 11, 13, 17, 19]
    }
    else {
        segmented_sieve_of_eratosthenes(ub, Some(cmp::min(segment_size, limit)))
    };

    let mut left: Vec<usize> = vec![0; seed_primes.len()];
    for (n, i) in seed_primes.iter().enumerate() {
        left[n] = 2*i;
    }

    for j in 0..limit / segment_size + 1 {
        let offset = segment_size * j;
        for (n, i) in seed_primes.iter().enumerate() {
            let mut x = left[n];
            while x < segment_size + offset {
                sieve[x - offset] = false;
                x += *i;
            }
            left[n] = x;
        }
        for n in (1..sieve.len()).step_by(2) {
            if n + offset > limit {
                break;
            }
            if sieve[n] {
                out.push(n + offset);
            }
        }
        for i in (1..segment_size).step_by(2) {
            sieve[i] = true;
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_primes() {
        let vec = sieve_of_eratosthenes(2_000_000);
        assert!(142913828922 == vec.iter().fold(0, |sum, x| sum + x));
    }
    #[test]
    fn sum_of_primes2() {
        let vec = segmented_sieve_of_eratosthenes(2_000_000, None);
        println!("{}", vec.iter().fold(0, |sum, x| sum + x));
        assert!(142913828922 == vec.iter().fold(0, |sum, x| sum + x));
    }

    #[test]
    fn including_last() {
        let vec = sieve_of_eratosthenes(19);
        assert_eq!(vec![2, 3, 5, 7, 11, 13, 17, 19], vec);
    }
}