mod solution;

/*
use std::fs::File;
use std::io::{BufRead, BufReader};
*/

pub struct Solution;

macro_rules! test_case {
    ($solution:ident: $($tc:tt, $value:expr,)*) => {
        $(
            #[test]
            fn $tc() {
                //input needs a type here, but cannot understand
                //2-d arrays with variable lengths. This means
                //the types need to be string parsed before they
                //can enter the test_case macro :(
                let (input, expected) = $value;
                let actual = Solution::$solution(
                    input.iter().map(|row| Vec::from(row)).collect()
                );
                assert_eq!(actual, Vec::from(expected));
            }
        )*
    }
}

/*
const PARAMS: usize = 2;
const TESTCASE_FILE: &str = "testcase.txt";


impl Solution {
    pub fn garbage_collection_str(garbage: &[&str], travel: &[i32]) -> i32 {
        Self::garbage_collection(
            garbage.iter()
                .map(|s| s.to_string())
                .collect(),
            travel.to_vec(),
        )
    }
}
*/


test_case!(find_diagonal_order: 
           tc0, ([[1,2,3],[4,5,6],[7,8,9]], [1,4,2,7,5,3,8,6,9]),
            tc1, ([[1,2,3,4,5],[6,7],[8],[9,10,11],[12,13,14,15,16]], [1,6,2,8,7,3,9,4,12,10,5,13,11,14,15,16]),
            tc2, ([[]], []),
        );

fn main() {
    /*
    let file = File::open(TESTCASE_FILE).expect(format!("find and open {} ", TESTCASE_FILE).as_str());

    let mut reader = BufReader::new(file).lines();

    let test_case_macro_input = (0..).map_while(|count| {
        format!("tc{}, (({} ), {}),", count, (PARAMS..0).zip(reader)
        .map(|(_, line)| format!("{}, ", line.expect("read parameter line")).as_str())
        .collect(), reader.next().expect("read result line")
        )
    }).collect();
    */
}
