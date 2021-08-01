use std::iter;

pub fn decode_iterative(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut decoded: Vec<i32> = Vec::with_capacity(encoded.len()+1);
    decoded.push(first);
    let mut decoded_digit: i32 = first;
    for encoded_digit in encoded {
        decoded_digit ^= encoded_digit;
        decoded.push(decoded_digit);
    }
    decoded
}

pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
    let mut last_decode = first;
    iter::once(first).chain(encoded.iter().map(|x| {
        last_decode ^= x;
        last_decode
    })).collect()
}

fn main() {
    let test_cases: Vec<(Vec<i32>, i32, Vec<i32>)> = vec![
        (vec![1,2,3], 1, vec![1,0,2,1]),
        (vec![6,2,7,3], 4, vec![4,2,0,7,4]),

    ];
    for (input, n, expected) in test_cases {
        let pass_input = input.clone();
        println!("Output: {:?}, Expected: {:?}", decode_iterative(input, n), expected);
        println!("Output: {:?}, Expected: {:?}", decode(pass_input, n), expected);
    }
}
