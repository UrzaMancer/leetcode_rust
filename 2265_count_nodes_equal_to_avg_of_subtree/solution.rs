// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
//
// 2 ms 100.00%
// 2.14 MB 100.00%

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn average_of_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Self::single_subtree_triple(&root).2
    }


// This would be better served as a struct method
// and returning a named struct data type instead of this
// amorphous tuple that's hard to work with
    fn single_subtree_triple(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
        if let Some(subroot) = node {

// The hardest part of these binary tree problems in rust
// (for me) is remembering tree traversal basics and manipulating
// Option and Rc<RefCell<<T>> types.
// This boils down to pattern matching option, using
// match, if let Some(x) = root, or while let Some(x) = root
// and remembering that finding None is useful work not to be ignored.
// Followed by __assigning__ x.borrow() on the exposed Rc<RefCell<>>
// and always handling the data interior to the function
// (Don't try to pass around anything without the Option, only use
//  Option<Rc<RefCell<TreeNode>> and &Option<...> of the same)
            let subnode = subroot.borrow();
            let (left_sum, left_node_count, left_raeq_count) = Self::single_subtree_triple(&subnode.left);
            let (right_sum, right_node_count, right_raeq_count) = Self::single_subtree_triple(&subnode.right);

            (left_sum + right_sum + subnode.val,
            left_node_count + right_node_count + 1,
            {
                if subnode.val == (left_sum + right_sum + subnode.val) / (left_node_count + right_node_count + 1) {
                    left_raeq_count + right_raeq_count + 1
                } else {
                    left_raeq_count + right_raeq_count
                }
            })
        } else {
            // empty leaf: no sum, count, or root avg equal
            (0, 0, 0)
        }
    }
}
