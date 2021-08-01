pub fn smaller_numbers_iterative(nums: Vec<i32>) -> Vec<i32> {
    let mut memo: Vec<i32> = vec![0; nums.len()];
    for (index, element) in nums.iter().enumerate() {
        let mut count: u16 = 0;
        for i in &nums {
            if i < element {
                count += 1;
            }
        }
        memo[index] = count as i32;
    }
    memo
}

pub fn smaller_numbers_memoized_iterative(nums: Vec<i32>) -> Vec<i32> {
    let mut sorted_nums = nums.clone(); 
    let mut memoization: Vec<i32> = vec![0; nums.len()]; 
    sorted_nums.sort_unstable(); 
    //println!("{:?}", sorted_nums);
    for (index, number) in sorted_nums.iter().enumerate().rev() {
        //println!("(index: {}, number: {})", index, number);
        for left in (0..index).rev() {
            //println!("left: {}, sorted_nums[{}]: {}", left, left, sorted_nums[left]);
            if &sorted_nums[left] != number {
                memoization[*number as usize] = left as i32 + 1;
                break;
            }
        }
    }
    nums.iter().map(|num| memoization[*num as usize]).collect()
}

pub fn smaller_numbers_with_map(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|a| {
        nums[..].iter().filter(|x| *x < a).count() as i32 
    }).collect::<Vec<i32>>()
}

pub fn smaller_numbers_memoized_closures(nums: Vec<i32>) -> Vec<i32> {
    let mut count_memo: Vec<Option<u16>> = vec![None; nums.len()];
    let mut sorted_nums = nums.clone();
    sorted_nums.sort_unstable();
    let mut memoize = |(i, x): (usize, &i32)| {
        let memo = i - sorted_nums[..i].iter().rev().take_while(|&n| n == x).count();
        count_memo[*x as usize] = Some(memo as u16);
        memo as u16
    };
    
    let check_count = |x: &i32| {
        match count_memo.get(*x as usize) {
            Some(Some(n)) => *n as i32,
            _ => 0,
        }
    };

    sorted_nums.iter().enumerate().rev().map(|(i, v)| {
        let memo = count_memo[*v as usize];
        match memo {
            Some(n)  => n,
            None => memoize((i, v)),
        }
    }).last();
    nums.iter().map(check_count).collect::<Vec<i32>>()
}

fn main() {
    let test_cases: Vec<(Vec<i32>, Vec<i32>)> = vec![
        (vec![6,5,4,8], vec![2,1,0,3]),
        (vec![7,7,7,7], vec![0,0,0,0]),
        (vec![36,28,40,50,89,49,85,99,22,1,49,83,44,77,100,99,78,28,29,8,78,40,49,26,90,37,12,55,0,44,11,52,19,14,68,93,22,96,43,18,48,55,41,69,81,89,66,35,26,24,20,21,68,20,18,15,56,74,66,16,55,17,42,77,93,89,87,40,76,17,2,45,99,22,28,30,88,63,75,62,4,83,11,17,68,60,72,18,26,46,90,17,35,94,44,8,49,60,3,67,59,25,92,30,26,88,0,92,89,38,5,34,66,24,9,13,98,14,51,42,13,57,91,73,66,90,83,61,85,35,18,55,47,82,44,96,49,52,55,47,70,26,25,8,18,30,30,2,30,85,50,58,100,78,56,72,12,6,22,67,0,5,55,100,35,53,16,30,88,98,84,47,53,99,43,74,95,6,64,58,16,88,66,46,74,7,44,15,31,81,81,15,55,19,2,46,16,19,36,3,60,81,3,61,8,96,75,55,11,60,69,77,64,12,16,68,86,90,50,82,77,69,52,30,66,48,61,93,40,56,14,8,26,75,78,89,47,13,12,45,13,25,14,88,83,75,11,0,65,55,30,11,39,62,78,7,73,54,71,38,50,80,5,85,76,3,35,79,61,56,28,34,41,99,61,56,68,7,34,65,22,95,90,56,58,37,74,75,23,41,65,84,50,97,85,88,52,87,71,76,50,61,96,80,28,39,2,8,18,63,14,46,33,39,69,38,17,13,64,17,58,4,57,60,93,72,41,69,47,16,91,1,68,25,98,34,79,8,100,59,36,1,82,34,92,64,88,2,72,51,14,12,87,41,77,79,81,89,88,73,57,11,78,63,67,43,75,86,5,24,21,7,78,14,5,15,71,67,39,96,100,25,20,68,19,29,27,11,64,45,88,72,55,34,78,17,36,73,31,8,76,33,7,2,55,78,36,60,54,55,8,23,27,98,86,4,81,96,54,53,45,45,65,18,22,28,29,7,46,8,57,41,27,28,32,19,88,90,66,48,13,61,87,93,100,78,0,60,39,33,99,89,37,26,16,88,81,48,84,4,100,98,48,58,68,3,16,29,63,82,12,37,89,79,55,25,66,42,12,31,31,62,73,14,22,63,45,90,82,76,81,79,55,51,70,53,96,3,80,51], vec![178,140,195,242,190,237,167,231,112,5,237,160,211,124,237,231,129,140,147,37,129,195,237,130,198,183,55,7,0,211,48,252,102,68,81,210,112,218,208,95,232,7,199,89,147,190,69,173,130,121,107,110,81,107,95,76,21,109,69,80,7,88,205,124,210,190,175,195,119,88,8,216,231,112,140,151,179,55,113,52,20,160,48,88,81,38,99,95,130,222,198,88,173,215,211,37,237,38,14,77,36,124,207,151,130,179,0,207,190,187,24,167,69,121,47,62,226,68,248,205,62,27,205,104,69,198,160,45,167,173,95,7,227,155,211,218,237,252,7,227,94,130,124,37,95,151,151,8,151,167,242,31,237,129,21,99,55,29,112,77,0,24,7,237,173,0,80,151,179,226,164,227,0,231,208,109,216,29,60,31,80,179,69,222,109,31,211,76,159,147,147,76,7,102,8,222,80,102,178,14,38,147,14,45,37,218,113,7,48,38,89,124,60,55,80,81,172,198,242,155,124,89,252,151,69,232,45,210,195,21,68,37,130,113,129,190,227,62,55,216,62,124,68,179,160,113,48,0,65,7,151,48,190,52,129,31,104,4,96,187,242,144,24,167,119,14,173,139,45,21,140,167,199,231,45,21,81,31,167,65,112,216,198,21,31,183,109,113,119,199,65,164,242,225,167,179,252,175,96,119,242,45,218,144,140,190,8,37,95,55,68,222,164,190,89,187,88,62,60,88,31,20,27,38,210,99,199,89,227,80,205,5,81,124,226,167,139,37,237,36,178,5,155,167,207,60,179,8,99,248,68,55,175,199,124,139,147,190,179,104,27,48,129,55,77,208,113,172,24,121,110,31,129,68,24,76,96,77,190,218,237,124,107,81,102,147,137,48,60,216,179,99,7,167,129,88,178,104,159,37,119,164,31,8,7,129,178,38,4,7,37,119,137,226,172,20,147,218,4,0,216,216,65,95,112,140,147,31,222,37,27,199,137,140,163,102,179,198,69,232,62,45,175,210,237,129,0,38,190,164,231,190,183,130,80,179,147,232,164,20,237,226,232,31,81,14,80,147,55,155,55,183,190,139,7,124,69,205,55,159,159,52,104,68,112,55,216,198,155,119,147,139,7,248,94,0,218,14,144,248])
    ];
    for (case, expected) in test_cases {
        println!("Output: {:?}     Expected: {:?}", smaller_numbers_iterative(case[..].to_vec()), expected);
        println!("Output: {:?}     Expected: {:?}", smaller_numbers_memoized_closures(case[..].to_vec()), expected);
        println!("Output: {:?}     Expected: {:?}", smaller_numbers_memoized_iterative(case[..].to_vec()), expected);
        println!("Output: {:?}     Expected: {:?}", smaller_numbers_with_map(case), expected);
    }
}
