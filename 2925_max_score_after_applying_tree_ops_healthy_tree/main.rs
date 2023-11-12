use Solution;
use std::fs::File;
use std::io::{self, BufRead};

struct Solution;

let const PARAMETER_COUNT: u8 = 2;

macro_rules! test_case {
    ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let actual = Solution::two_sum(input.0, input.1);
                assert_eq!(actual, expected);
            }
        )*
    }

fn main() {
    let test_case_file = fs::open("test_cases.txt")?;
    let mut count: usize = 0;
    io::BufReader::new(test_case_file)
        .lines()
        .windows(PARAMETER_COUNT)
        .for_each(|parametrray| {
            test_case!(test_case_{count}: parametrray);
        });
}

