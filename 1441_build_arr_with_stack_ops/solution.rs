// 0 ms 100.00%
// 2.11 MB 34.62%
impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut operations: Vec<String> = Vec::new();

        let mut stream_cursor: i32 = 1;

        // I think we would like to use flat map here to collect
        // the sequences of stack operations rather than messing
        // with the Vec of strings
        target.iter().for_each(|&num| {
            while num != stream_cursor {
                // "Push" op and stream_cursor increment must go together
                // which is a good indication this should be a function
                // or perhaps... struct method
                operations.push("Push".to_string());
                operations.push("Pop".to_string());
                stream_cursor += 1;
                // stream_cursor is also sort of silly as it only increments
                // which should lend itself to an iterator pretty easily
            }
            operations.push("Push".to_string());
            stream_cursor += 1;
        });

        operations
    }
}
