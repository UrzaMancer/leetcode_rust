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
                   Vec::from(input.0), Vec::from(input.1), Vec::from(input.2) 
                );
                assert_eq!(actual, expected); 
            }
        )*
    }
}

fn generate_testcase(ct: usize, constraint_ranges: &mut Vec<std::ops::RangeInclusive<i32>>)
    -> (String, (Vec<i32>, Vec<i32>, Vec<i32>)) {
    let mut rng = thread_rng();
    let (n, m) = (rng.gen_range(constraint_ranges[0].clone()), rng.gen_range(constraint_ranges[1].clone()));
    let nums: Vec<i32> = (0..n).map(|_| rng.gen_range(constraint_ranges[2].clone())).collect();
    let r: Vec<i32> = (0..m).map(|_| rng.gen_range(1..n)).collect();
    let l: Vec<i32> = r.iter().map(|&right| rng.gen_range(0..right)).collect();
    (
        format!("tc{}", ct).to_string(), 
        (nums, l, r)
    )
}


test_case!(check_arithmetic_subarrays: 
           tc0, (([4,6,5,9,3,7], [0,0,2], [2,3,5]), [true,false,true]),
            tc1, (([-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10], [0,1,6,4,8,7], [4,4,9,7,9,10]), [false,true,false,false,true,true]),
        );


fn main() {
    (0..10).for_each(|count| {
        let (_, (nums, l, r)) = 
        generate_testcase(count+2, &mut vec![2..=500, 1..=500, -100_000..=100_000]);
        println!("{:?}\n{:?}\n{:?}\n", nums, l, r);
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
