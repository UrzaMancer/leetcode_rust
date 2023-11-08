# Intuition
The player may move in any of the 8 directions, so the max range of moves is given by the delta (absolute value difference) between the start and finish positions for both x and y coordinates. As long as the delta in both dimensions is less than or equal to t, the max position is reachable.

# Constraints
    1 <= sx, sy, fx, fy <= 109
    0 <= t <= 109

# Approach
Naively, we can compare `fx` to `sx + t` and `sx - t` and make sure these are all equal or less than. However, we need two comparisons for each pair of values. This is not give up any meaningful computational efficiency, but can be written more neatly. 

Additionally, the player must move at each second. While inefficient moves are possible to reach odd position deltas at even times, and so forth, the exception is at 1 second, when the origin is not reachable. We include a special test for this special case.

To address code repetition and exhaustive testing, we implement a match statement using std::cmp::Ordering, capturing significant elements of false conditions together, evaluating the special case of `t == 1`, and true for the rest.

# My Quirks / Lessons Learned

I had 3 failed submissions: `Wrong Answer`, which seems like a lot for a problem that appears this surface level (See Useful Test Cases). I absolutely missed the significance of the "player must move" requirement and t == 1. After seeing the result, I picked it up quickly, but then assumed without sufficient analysis that it would be a similar problem for all odd `t`, leading to the second failed submission.

I submitted successfully with my initial basic boolean chain code, then read some other solutions in Rust to see if I missed anything novel or obvious. I picked up using the `.abs()` calculation to reduce code from:

* [SalvadorDali's Rust/Python O(1) with detailed explanation](https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/solutions/4024850/rust-python-o-1-with-detailed-explanation/)

* [rony0000013's Rust 0ms 🔥 One Liner 🦀](https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/solutions/4264652/rust-0ms-one-liner/)

and realized the chaining comparisons could be written best in Rust using `match` with `Ordering` to enable exhaustive search, something I had played with before but didn't feel I understood super well. (It always bothers me when I think to match one or more boolean expressions like `match sx - fx <= t { true => ..., false => ... }` since you may want to handle all 3 comparison cases instead of two)

Finally, the last failed submission was with the refactored code, but the middle branch evaluated on either delta less than `t`:
```rust
(Ordering::Less, _) | (_, Ordering::Less) => t != 1,
```
It needed to evaluate on both instead

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

# Author
Created 2023-11-08
by UrzaMancer
[LeetCode ](https://leetcode.com/UrzaMancer/)
[GitHub](https://github.com/UrzaMancer)
[LeetCode Solution Post](https://leetcode.com/problems/determine-if-a-cell-is-reachable-at-a-given-time/post-solution/4265439/)

![LeetCode](https://assets.leetcode.com/static_assets/others/LeetCode_logo_black.png){:style='width:25px'}
