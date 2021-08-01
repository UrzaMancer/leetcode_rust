pub fn restore_string_into_chars_vec(s: String, indices: Vec<i32>) -> String {
    let mut ordered_characters: Vec<char> = vec!['A'; indices.len()];
    for (i, c) in indices.iter().zip(s.chars()) {
        ordered_characters[*i as usize] = c;
    }
    ordered_characters.into_iter().collect()
}

pub fn restore_string_with_replace_range(s: String, indices: Vec<i32>) -> String {
    let mut ordered_string: String = s.clone();
    println!("{}", ordered_string.capacity());
    for (i, c) in indices.iter().zip(s.chars()) {
        println!("Attempting to insert {} at {}", c, i);
        ordered_string.replace_range((*i as usize)..=(*i as usize), &c.to_string()[..]);
    }
    ordered_string
}

pub fn restore_string_with_remove_and_insert(s: String, indices: Vec<i32>) -> String {
    let mut ordered_string: String = s.clone();
    println!("{}", ordered_string.capacity());
    for (i, c) in indices.iter().zip(s.chars()) {
        println!("Attempting to insert {} at {}", c, i);
        ordered_string.remove(*i as usize);
        ordered_string.insert(*i as usize, c);
    }
    ordered_string
}

fn main() {
    let test_cases: Vec<(&str, Vec<i32>, &str)> = vec![
        ("codeleet", vec![4,5,6,7,0,2,1,3], "leetcode"),
        ("abc", vec![0,1,2], "abc"),
        ("aiohn", vec![3,1,4,2,0], "nihao"),
        ("aaiougrt", vec![4,0,2,6,7,3,1,5], "arigatou"),
        ("art", vec![1,0,2], "rat"),
    ];
    for (s, indices, expected) in test_cases {
        println!("Output: {}, Expected: {}", restore_string_into_chars_vec(s.to_string(), indices), expected);
    }

}
