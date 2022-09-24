#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use integer_sqrt::IntegerSquareRoot;

fn main() {
    println!("Problem 3: {}", problem_3(600_851_475_143));
}

/*
 * Problem 3: Largest prime factor
 * 
 * The prime factors of 13195 are 5, 7, 13 and 29.
 *
 * What is the largest prime factor of the number 600851475143 ?
 */
fn problem_3(target: usize) -> usize {
    // Unwrapping should be safe since `target.integer_sqrt()` should get us enough primes.
    #[allow(clippy::unwrap_used)]
    let factors = primal::Sieve::new(target.integer_sqrt())
        .factor(target)
        .unwrap();
    let (biggest_prime, _) = factors[factors.len()-1];
    biggest_prime
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_3_test() {
        assert_eq!(problem_3(13195), 29);
    }
}