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
//
// 0 ms 100%
// 3.06 MB 70.00%

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::cmp::max;

impl Solution {
    pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut modes: Vec<i32> = Vec::new();
        let mut count_map: HashMap<i32, i32> = HashMap::new();
        let mut max_count: i32 = 1;
        Self::traverse(&root, &mut count_map, &mut max_count);
        count_map.iter()
            .filter_map(|(&value, count)| {
                if count == &max_count {
                    Some(value)
                } else {
                    None
                }
            })
            .collect::<Vec<i32>>()
    }

    fn traverse(next_node: &Option<Rc<RefCell<TreeNode>>>, count_map: &mut HashMap<i32, i32>, max_count: &mut i32) {
        if let Some(node_cell) = next_node {
            let node = node_cell.borrow();
            *count_map.entry(node.val)
                .and_modify(|e| {
                    *e += 1;
                    *max_count = max(*max_count, *e);
                })
                .or_insert(1);

            Self::traverse(&node.left, count_map, max_count);
            Self::traverse(&node.right, count_map, max_count);
        }
    }

}
