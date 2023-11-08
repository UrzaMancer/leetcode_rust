/*
# Author
Created 2023-11-08
by UrzaMancer
[LeetCode ](https://leetcode.com/UrzaMancer/)
[GitHub](https://github.com/UrzaMancer)
[LeetCode Solution Post](https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/post-solution/4265439/)
                               
# Complexity
### Time complexity: $O(1)$
> There are always 9 or 10 operations for every input.

### Space complexity: $O(1)$
> We use two (optional) variables to store the deltas, which is the only additional memory beyond the input, and is constant.

# Submission Result
[0 ms => 100.00%
1.95 MB => 77.27%](https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/submissions/1094560656?envType=daily-question&envId=2023-11-08)

# Useful TestCases
> _Note_: I don't know why this solution test harness requires one integer argument per line, but I tested separating with spaces and it does not run. This seems like a bug.

| case A | case B | case C |
| --- | --- | --- |
|1<br>4<br>1<br>3<br>1|1<br>1<br>1<br>1<br>3|1<br>2<br>1<br>2<br>1|
|`true`|`true`|`false`|

*/
                               
use std::cmp::Ordering;

impl Solution {
    pub fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
        let delta_x: i32 = (sx - fx).abs();
        let delta_y: i32 = (sy - fy).abs();
        match (delta_x.cmp(&t), delta_y.cmp(&t)) {
            (Ordering::Greater, _) | (_, Ordering::Greater) => false,
            (Ordering::Less, Ordering::Less) => t != 1,
            _ => true,
        }
    }
}
