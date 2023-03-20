#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::cmp::min;

use anyhow::{Context, Result};

/*
A palindromic number reads the same both ways. The largest palindrome made from the product of
two 2-digit numbers is 9009 = 91 × 99.

Find the largest palindrome made from the product of two 3-digit numbers.
 */

fn main() -> Result<()> {
    println!("Problem 4: {}", problem_4(3)?);
    // println!("Problem 4: {}", problem_4_simple());
    Ok(())
}

// fn problem_4_simple() -> u32 {
//     let mut largest = u32::MIN;
//     for x in 0..=999 {
//         for y in x..=999 {
//             let p = x * y;
//             let s = p.to_string();
//             if p > largest && is_palindrome(&s) {
//                 // println!("New largest {p} from ({x}, {y}).");
//                 largest = p;
//             }
//         }
//     }
//     largest
// }

struct PairIter {
    max_value: u32,
    x: u32,
    // y is current_sum - x
    current_sum: u32
}

impl PairIter {
    const fn new(max_value: u32) -> Self {
        Self {
            max_value,
            x: 0,
            current_sum: 2 * max_value + 1
        }
    }
}

/// We'll go through all the pairs where `0≤x<max_value`
/// and `0≤y<max_value` by decreasing sums. For a give sum
/// we'll go from from (sum, 0) to (sum-1, 1), etc., to (0, sum).
/// All pairs will be bounded between 0 and `max_value`, though,
/// so we usually won't be starting or ending at (0, sum) or
/// (sum, 0).
impl Iterator for PairIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_sum == 0 {
            return None;
        }
        let x = i64::from(self.x) - 1;
        let y = i64::from(self.current_sum) - x;
        if x < 0 || y > i64::from(self.max_value) {
            self.current_sum -= 1;
            self.x = min(self.max_value, self.current_sum);
        } else {
            self.x -= 1;
        }
        Some((self.x, self.current_sum - self.x))
    }
}

/*
 * Problem 4: Largest palindrome product
 * 
 * A palindromic number reads the same both ways. The largest palindrome
 * made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * 
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */
fn problem_4(num_digits: u32) -> Result<u32> {
    // Assuming we don't have more than 3 digits, this should easily fit in a `u32`.
    #[allow(clippy::cast_possible_truncation)]
    let max_value = (10_usize.pow(num_digits) - 1) as u32;
    let iter = PairIter::new(max_value);

    // This should find at least one palindrome, so the unwrap() should be safe.
    // I could bring in the anyhow tools to avoid this wrap, though.
    #[allow(clippy::unwrap_used)]
    iter // .inspect(|(x, y)| println!("({x}, {y})"))
        .map(|(x, y)| x * y)
        .find(|p| is_palindrome(&p.to_string()))
        .context("We never found a palindrome")
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unwrap_used)]
    fn iter_test() {
        let mut iter = PairIter::new(4);
        let (x, y) = iter.next().unwrap();
        assert_eq!(4, x, "x should have been 4");
        assert_eq!(4, y, "y should have been 4");

        let (x, y) = iter.next().unwrap();
        assert_eq!(4, x);
        assert_eq!(3, y);

        let (x, y) = iter.next().unwrap();
        assert_eq!(3, x);
        assert_eq!(4, y);

        let (x, y) = iter.next().unwrap();
        assert_eq!(4, x);
        assert_eq!(2, y);

        let (x, y) = iter.next().unwrap();
        assert_eq!(3, x);
        assert_eq!(3, y);
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn problem_4_test() {
        assert_eq!(problem_4(2).unwrap(), 9009);
    }
}