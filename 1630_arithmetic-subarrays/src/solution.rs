use crate::Solution;

impl Solution {
    pub fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
        l.iter().zip(r.iter())
            .map(|(&left, &right)| &nums[left as usize..=right as usize])
            .map(|range|
                 Self::is_arithmetic(range)
                 || Self::is_arithmetic(&Self::sort_range(range))
            ).collect()
    }

    fn sort_range(range: &[i32]) -> Vec<i32> {
        let mut sorted_vec: Vec<i32> = range.to_vec();
        sorted_vec.sort_unstable();
        sorted_vec
    }

    fn is_arithmetic(range: &[i32]) -> bool {
        print!("{:?}", range);
        range.len() > 1 &&
        range.iter().zip(range[1..].iter())
            .map(|(&prev, &num)| num - prev)
            .fold((true, None), |(equal_diff, last): (bool, Option<i32>), diff|
                  (
                      equal_diff 
                      && (last.is_none()
                      || last.unwrap() == diff),
                      Some(diff)
                    )
                  ).0
    }
}
