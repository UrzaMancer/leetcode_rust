pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums[..].iter().enumerate().fold(vec![0, 0], |tup_acc, (i, &a)| {
        match nums[..].into_iter().enumerate().rev()
        .find(|(j, &b)| a + b == target && i != *j) {
            Some((j, _)) => vec![i as i32, j as i32],
            None => tup_acc,
        }}) 
}

pub fn two_sum_opt(nums: Vec<i32>, target: i32) -> Vec<i32> {
    nums[..].iter().enumerate().map(|(i, &a)| {
        let j = nums[i+1..].iter().position(|&x| x == target - a);
        if let Some(index) = j {
            vec![i as i32, (index+i+1) as i32]
        } else { vec![] }
    }).skip_while(|x| x.is_empty() ).next().unwrap()
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![2,7,11,15], 9, vec![0,1]),
        (vec![3,2,4], 6, vec![1,2]),
        (vec![3,3], 6, vec![0,1]),
    ];
    for (case_nums, case_target, expected) in test_cases {
        println!("Input: {:?} {} |-> Output: {:?} =?= Expected: {:?}", case_nums, case_target, two_sum(case_nums[..].to_vec(), case_target), expected);
        println!("Input: {:?} {} |-> Output: {:?} =?= Expected: {:?}", case_nums, case_target, two_sum_opt(case_nums[..].to_vec(), case_target), expected);
    }
}
