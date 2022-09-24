#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

use std::cmp::min;

fn main() {
    println!("Problem 4: {}", problem_4(3));
}

struct Pair {
    max_value: usize,
    x: usize,
    current_sum: usize
}

impl Pair {
    fn new(max_value: usize) -> Self {
        Pair {
            max_value,
            x: max_value,
            current_sum: 2 * max_value
        }
    }

    fn pair(&self) -> (usize, usize) {
        (self.x, self.current_sum-self.x)
    }
}

/// We'll go through all the pairs where 0≤x<max_value
/// and 0≤y<max_value by decreasing sums. For a give sum
/// we'll go from from (0, sum) to (1, sum-1), etc., to (sum, 0).
/// All pairs will be bounded between 0 and max_value, though,
/// so we usually won't be starting or ending at (0, sum) or
/// (sum, 0). So, for a given sum, we'll start and end at
/// m = min(sum, |sum-max_value|), i.e., go from (m, sum-m) to
/// (sum-m, m).
impl Iterator for Pair {
    type Item = Pair;

    fn next(&mut self) -> Option<Pair> {
        if self.current_sum == 0 {
            return None;
        }
        if self.x >= self.current_sum {
            self.current_sum = self.current_sum - 1;
            let m = (self.current_sum as i64 - self.max_value as i64).abs() as usize;
            self.x = min(self.current_sum, m);
        } else {
            self.x = self.x + 1;
        }
        Some(Pair { max_value: self.max_value, x: self.x, current_sum: self.current_sum })
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
fn problem_4(num_digits: u32) -> usize {
    let max_value = 10_usize.pow(num_digits) - 1;
    let iter = Pair::new(max_value);
    

    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_test() {
        let iter = Pair::new(4);
        let (x, y) = iter.pair();
        assert_eq!(2, x, "x should have been 2");
        assert_eq!(2, y, "y should have been 2");
    }

    #[test]
    fn problem_4_test() {
        assert_eq!(problem_4(2), 9009);
    }
}