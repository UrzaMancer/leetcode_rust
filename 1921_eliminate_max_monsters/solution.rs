// 33 ms 16.67%
// 3.44 MB 66.67%


impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let n: usize = dist.len();
        let mut time_to_reach: Vec<f32> =
            dist.iter().zip(speed.iter())
                .map(|(&s, &d)| s as f32 / d as f32)
        // using f32 here may not be best,
        // but it was extremely easy for me to code this
        // using this time_to_reach paradigm.
                .collect();
        
        time_to_reach.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // Alternatively, to avoid floats and partial comparison,
        // We could track gamestate and count by ticks, which also
        // seems like an interesting solution.

        (0..n).find(|&minutes_passed|
            minutes_passed as f32 >= time_to_reach[minutes_passed]
            )
            .unwrap_or(n) as i32
    }
}
