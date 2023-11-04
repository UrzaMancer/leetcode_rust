// https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/
// 0 ms 100.00%
// 2.08 MB 100.00%
//
// Collisions are the same as ants passing through, so we can just
// calculate the time for each ant to fall off the plank and take the
// maximum.

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        // I like the chain iterator method better here, but
        // I did not win my fight against the borrow checker.
        // After using map I seem to violate some trait bound that
        // breaks the chain.
        std::cmp::max(
            *left.iter().max().unwrap_or(&0),
            right.iter().map(|&lpos| n - lpos).max().unwrap_or(0)
        // We also prefer something more elegant than unwrap, but
        // this is functional for a programming challenge problem
        )
    }
}
