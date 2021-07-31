use std::char;
pub fn maximum_number(num: String, change: Vec<i32>) -> String {
    let mut max: String = String::from(""); 
    let mut in_sub_string: bool = false;
    struct SubBoundaries {
        start: usize,
        end: usize,
    }
    let mut sub_boundaries: SubBoundaries = SubBoundaries { start: num.len(), end: num.len()};
    let mut sub_string = String::from("");
    for (index, single) in num.chars().enumerate() {
        let digit: usize = match single.to_digit(10) {
            Some(n) => n as usize,
            None => 0,
        };
        if digit as i32 > change[digit] {
            if in_sub_string {
                sub_boundaries.end = index;
                break;
            }
            continue;
        }
        if digit as i32 == change[digit] && !in_sub_string {
            continue;
        }
        if let Some(changed) = char::from_digit(change[digit] as u32, 10) {
            sub_string.push(changed);
        }
        //println!("{:?}", num);
        if !in_sub_string {
           sub_boundaries.start = index; 
           in_sub_string = true;
           continue;
        } 
    }
    max.push_str(&num[..sub_boundaries.start]);
    max.push_str(&sub_string);
    max.push_str(&num[sub_boundaries.end..]);
    max
}

pub fn maximum_number_with_closures(num: String, change: Vec<i32>) -> String {
    let mut in_string: Option<bool> = None;
    let digitized = |a: char| -> i32 {
        match a.to_digit(10) { Some(n) => n as i32, None => 10,}
    };
    let mutated = |x: char| -> i32 {
        change[digitized(x) as usize]
    };
    let charitized = |i: i32| -> char {
        if let Some(character) = char::from_digit(i as u32, 10) { character } else { panic!("invalid digit!"); }
    };
    let mut track_sub_string = |a, b| -> i32 {
        if b > a {
            match in_string {
                Some(true) => b,
                Some(false) => a,
                None => { in_string = Some(true); b },
            }
        } else if b < a {
          match in_string {
              Some(true) => { in_string = Some(false); a },
              _ => a,
          }  
        } else { a }
    };
    let zipped_mutate = num.chars().map(|d| digitized(d)).zip(num.chars().map(|c| mutated(c))).collect::<Vec<(i32, i32)>>();
    //println!("{:?}", zipped_mutate);
    let filtered_zip: Vec<i32> = zipped_mutate.iter().map(|(digit, mutated)| track_sub_string(*digit, *mutated)).collect();
    //println!("{:?}", filtered_zip);
    filtered_zip.iter().map(|i| charitized(*i)).collect()
}
fn main() {
    let test_cases: Vec<(String, Vec<i32>, String)> = vec![
        ("132".to_string(), vec![9,8,5,0,3,6,4,2,6,8], "832".to_string()),
        ("021".to_string(), vec![9,4,3,5,7,2,1,9,0,6], "934".to_string()),
        ("5".to_string(), vec![1,4,7,5,3,2,5,6,9,4], "5".to_string()),
        ("334111".to_string(), vec![0,9,2,3,3,2,5,5,5,5], "334999".to_string()),
    ];
    for (a, b, expected) in test_cases {
        println!("Output: {:?}     Expected: {:?}", maximum_number(a.clone(), b.clone()), expected);
        println!("maximum_number_with_closures -- Output: {:?}  Expected: {:?}", maximum_number_with_closures(a.clone(), b.clone()), expected);
    }
}
