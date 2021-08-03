pub fn decompress_rl_elist_iterative(nums: Vec<i32>) -> Vec<i32> {
    //let mut result = Vec::new();
    nums.iter().step_by(2).zip(nums[1..].iter().step_by(2)).flat_map(|(&freq, &val)| vec![val; freq as usize]).collect()
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
        (vec![1,2,3,4], vec![2,4,4,4]),
        (vec![1,1,2,3], vec![1,3,3]),
    ];
    for (case, expected) in test_cases {
        println!("Output: {:?}     Expected: {:?}", decompress_rl_elist_iterative(case[..].to_vec()), expected);
    //    println!("Output: {:?}     Expected: {:?}", smaller_numbers_memoized_closures(case[..].to_vec()), expected);
    //    println!("Output: {:?}     Expected: {:?}", smaller_numbers_memoized_iterative(case[..].to_vec()), expected);
    //    println!("Output: {:?}     Expected: {:?}", smaller_numbers_with_map(case), expected);
    }
}
