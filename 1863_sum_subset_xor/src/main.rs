pub fn powerset(items: &[i32]) -> Vec<Vec<&i32>> {
    (0..2usize.pow(items.len() as u32)).map(|count| {
        items.iter().enumerate().filter(|&(t, _)| (count >> t) % 2 == 1)
                                .map(|(_, element)| element)
                                .collect()
        }).collect()
}

pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
    let mut counter = 0;
    let mut xor_total_sum = 0;
    for subset in powerset(&nums) {
        let mut xor_total;
        match subset.first() {
            Some(0) => continue,
            Some(i) => xor_total = **i,
            None => continue,
        };
        println!("subset {}: {:?}", counter, subset);
        println!("  *Starting xor_total:{}", xor_total);
        for element in &subset[1..] {
            xor_total ^= **element;
            println!("  *New xor_total: {}", xor_total);
        }
        xor_total_sum += xor_total;
        println!(" -xor_total_sum: {}", xor_total_sum);
        counter += 1;
    }
    return xor_total_sum;
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32)> = vec![
        (vec![1,3], 6),
        (vec![5,1,6], 28),
        (vec![3,4,5,6,7,8], 480),
    ];
    for (case, expected) in test_cases {
        println!("Output: {}     Expected: {}", subset_xor_sum(case), expected);
    }
}
