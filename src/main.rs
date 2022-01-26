use rayon::prelude::*;
use std::collections::HashMap;

const LIMIT: u64 = 1_000_000;

fn main() {
    // collect the liars up to the limit
    let (elem, count) = (0..=LIMIT)
        .into_par_iter()
        .flat_map(|n| {
            // use star witnesses to asses primality
            let is_prime = small_n_is_prime(n);
            (2..n).into_par_iter().filter(move |&a| {
                // determine if witness a is a liar
                miller_rabin(n, a) & !is_prime
            })
        })
        .fold(Default::default, |mut acc: HashMap<u64, usize>, elem| {
            *acc.entry(elem).or_default() += 1;
            acc
        })
        .reduce(Default::default, |a, b| {
            let (mut max, min) = if a.len() > b.len() {
                (a, b)
            } else {
                (b, a)
            };

            for (elem, count) in min {
                *max.entry(elem).or_default() += count;
            }

            max
        })
        .into_par_iter()
        .max_by_key(|(_, count)| *count)
        .unwrap();

    println!("{}: {}", elem, count);
}

fn miller_rabin(n: u64, a: u64) -> bool {
    fn split_n(n: u64) -> (u64, u64) {
        let x = n - 1;
        let s = x.trailing_zeros() as u64;
        let d = x / (1 << s);

        (s, d)
    }

    fn modpow(mut base: u64, mut exponent: u64, modulus: u64) -> u64 {
        if modulus == 1 {
            return 0;
        }

        let mut res = 1;

        base = base % modulus;
        while exponent > 0 {
            if exponent % 2 == 1 {
                res = (res * base) % modulus;
            }
            exponent >>= 1;
            base = (base * base) % modulus;
        }

        res
    }

    // if its less than two or even then we can just compare with 2
    if n < 2 || n % 2 == 0 {
        return n == 2;
    }

    let (s, mut d) = split_n(n);
    for _ in 0..s {
        let witness = modpow(a, d, n);
        if witness == 1 || witness == n - 1 {
            return true;
        }

        d *= 2;
    }

    false
}

fn small_n_is_prime(n: u64) -> bool {
    if n <= 2 {
        return n == 2;
    }

    let star_witnesses = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];
    star_witnesses.into_iter().filter(|a| a < &n).all(|a| miller_rabin(n, a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miller_rabin() {
        assert!(!miller_rabin(747, 23));
        assert!( miller_rabin(221, 174));
        assert!(!miller_rabin(221, 137));
    }
}
