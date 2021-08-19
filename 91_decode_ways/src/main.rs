pub fn bad_num_decodings(s: String) -> i32 {
    if s.as_bytes().first() == Some(&b'0') { return 0; }
    if s.len() <= 1 { return s.len() as i32; }
    let last_two = s.as_bytes().rchunks_exact(2).next().unwrap();
    s.as_bytes()
    .windows(3)
    //.inspect(|x| println!("{:?}", x.to_ascii_lowercase()))
    .fold(1, |acc, window| {
        acc + match window.first() {
            Some(b'0') => match window.get(1) { Some(b'0') => { return 0; }, _ => 0 },
            Some(b'1') => match window.get(1) {
                        Some(b'0') => 0,
                        Some(b'1') | Some(b'2') => match window.last() { Some(b'0') => 0, _ => 1 },
                        _ => 1
                        },
            Some(b'2') => match window.get(1) {
                        Some(b'3') | Some(b'4') | Some(b'5') | Some(b'6') => 1,
                        Some(b'1') | Some(b'2') => match window.last() { Some(b'0') => 0, _ => 1 },
                        _ => 0
                        },
            _ => match window.get(1) { Some(b'0') => { return 0; }, _ => 1 },
        }
    }) + match last_two.first().unwrap() {
        b'0' => match last_two.last().unwrap() { b'0' => { return 0; }, _ => 0 },
        b'1' => match last_two.last().unwrap() { b'0' => 0, _ => 1 },
        b'2' => match last_two.last().unwrap() { b'0' | b'7' | b'8' | b'9' => 0, _ => 1 },
        _ => match last_two.last().unwrap() { b'0' => { return 0; }, _ => 0 }
    }
}

fn main() {
    let test_cases: Vec<(&str, i32)> = vec![
        ("12", 2), 
        ("0", 0),
        ("1", 1),
        ("10", 1),
        ("2101", 1),
        ("1123", 5),
    ];
    for (s, expected) in test_cases {
        println!("Input: {:?} |-> Output: {:?} =?= Expected: {:?}", s, bad_num_decodings(s.to_string()), expected);
    }
}
