use crate::Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        if nums.len() | l.len() | r.len() == 0 {
            return vec![];
        }
        vec![false]
    }
}
