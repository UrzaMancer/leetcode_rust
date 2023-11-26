mod solution;
use rand::{thread_rng, Rng};

pub struct Solution;

macro_rules! test_case {
    ($solution:ident: $($tc:tt, $value:expr,)*) => {
        $(
            #[test]
            fn $tc() {
                let (input, expected) = $value;
                let actual = Solution::$solution(
                    input.iter().map(|row| row.iter().map(|&x| x).collect()).collect()
                );
                assert_eq!(actual, expected); 
            }
        )*
    }
}

/* Using _ for array lengths is unstable - not sure how I can do this :(
fn vec_vec(nums: [[i32; _]; _]) -> Vec<Vec<i32>> {
    nums.iter().map(|row| row.iter().map(|&x| x).collect()).collect()
}
*/

fn generate_testcase(ct: usize, constraint_ranges: &mut Vec<std::ops::RangeInclusive<i32>>)
    -> (String, Vec<Vec<i32>>) {
    let mut rng = thread_rng();
    let n = rng.gen_range(constraint_ranges[0].clone());
    let m = rng.gen_range(1..=(100_000/n));
    let nums: Vec<Vec<i32>> = 
        (0..n).map(|_| 
                (0..m).map(|_| rng.gen_range(constraint_ranges[1].clone())).collect()
                ).collect();
    (
        format!("tc{}", ct).to_string(), 
        (nums)
    )
}


test_case!(largest_submatrix: 
           tc0, ([[0,0,1],[1,1,1],[1,0,1]], 4),
            tc1, ([[1,0,1,0,1]], 3),
            tc2, ([[1,1,0],[1,0,1]], 2),
        );


fn main() {
    (0..7).for_each(|count| {
        let (_, nums) = 
        generate_testcase(count+2, &mut vec![1..=100_000, 0..=1]);
        println!("{:?}", nums);
    });

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
