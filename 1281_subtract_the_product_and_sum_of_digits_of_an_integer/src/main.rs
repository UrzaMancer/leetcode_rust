pub fn subtract_product_and_sum(n: i32) -> i32 {
    let sum: i32 = n.to_string().chars().map(|x| match x.to_digit(10) { Some(i) => i as i32, None => 0i32}).sum();
    let product: i32 = n.to_string().chars().map(|x| match x.to_digit(10) { Some(i) => i as i32, None => 0i32}).product();
    product - sum
}

pub fn subtract_product_and_sum_inplace(n: i32) -> i32 {
    n.to_string().chars().map(|x| match x.to_digit(10) { Some(i) => i as i32, None => 0i32}).product::<i32>() - n.to_string().chars().map(|x| match x.to_digit(10) { Some(i) => i as i32, None => 0i32}).sum::<i32>()
}
fn main() {
    let test_cases: Vec<(i32, i32)> = vec![
        (234, 15),
        (4421, 21),
        (12, -1)
    ];
    for (n, expected) in test_cases {
        println!("Output: {}, Expected: {}", subtract_product_and_sum(n), expected);
        println!("Output: {}, Expected: {}", subtract_product_and_sum_inplace(n), expected);
    }
}
