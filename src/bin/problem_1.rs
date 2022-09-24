#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

fn main() {
    println!("Problem 1: {}", problem_1(1000));
}

/*
 * Problem 1: Multiple of 3 or 5
 * 
 * If we list all the natural numbers below 10 that are multiples of 3 or 5,
 * we get 3, 5, 6 and 9. The sum of these multiples is 23.
 *
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */
fn problem_1(limit: usize) -> usize {
    (1..limit)
        .filter(|n| n % 3 == 0 || n % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_1_test() {
        assert_eq!(problem_1(10), 23);
    }
}