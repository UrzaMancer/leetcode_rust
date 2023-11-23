mod solution;

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


test_case!(check_arithmetic_subarrays: 
           tc0, (([4,6,5,9,3,7], [0,0,2], [2,3,5]), [true,false,true]),
            tc1, (([-12,-9,-3,-12,-6,15,20,-25,-20,-15,-10], [0,1,6,4,8,7], [4,4,9,7,9,10]), [false,true,false,false,true,true]),
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
