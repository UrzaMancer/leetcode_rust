pub fn get_lucky(s: String, k: i32) -> i32 {
    let mut digits = string_to_int(s);
    for _n in 0..k {
        digits = sum_string_digits(digits).to_string();
    }
    sum_string_digits(digits)
}

pub fn sum_string_digits(s: String) -> i32 {
    let mut sum = 0;
    for num in s.chars() {
        sum += match num.to_digit(10) {
            Some(n) => n as i32,
            None => 0i32,
        }
    }
    println!("{}", sum);
    sum
}

pub fn string_to_int_bad(s: String) -> String {
    let digit = match &s[..] {
        "a" => "1",
        "b" => "2",
        "c" => "3",
        "d" => "4",
        "e" => "5",
        "f" => "6",
        "g" => "7",
        "h" => "8",
        "i" => "9",
        "j" => "10",
        "k" => "11",
        "l" => "12",
        "m" => "13",
        "n" => "14",
        "o" => "15",
        "p" => "16",
        "q" => "17",
        "r" => "18",
        "s" => "19",
        "t" => "20",
        "u" => "21",
        "v" => "22",
        "w" => "23",
        "x" => "24",
        "y" => "25",
        "z" => "26",
        _ => "",
    };
    digit.to_string()
}

pub fn string_to_int(s: String) -> String {
    let mut digit_string = String::from("");
    for digit in s.bytes() {
        digit_string.push_str(&(digit - 96).to_string());
    }
    println!("{:?}", digit_string);
    digit_string
}

fn main() {
    let test_cases: Vec<(String, i32, i32)> = vec![
        ("iiii".to_string(), 0, 36),
        ("leetcode".to_string(), 1, 6),
        ("zbax".to_string(), 1, 8),
    ];
    for (s, k, expected) in test_cases {
        println!("Output: {:?}     Expected: {:?}", get_lucky(s, k), expected);
    }
}
