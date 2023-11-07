use std::collections::HashMap;

#[derive(Debug)]
struct TreeLevel {
    value: i32,
    child_sum: i64,
    children: Vec<i32>,
}

impl TreeLevel {
    pub fn new(node: &i32, child: &i32, values: &Vec<i32>) -> Self {
        Self {
            value: values[*node as usize],
            child_sum: values[*child as usize] as i64,
            children: vec![*child],
        }
    }
    
    pub fn add_child(&mut self, child: &i32, values: &Vec<i32>) {
        
        self.child_sum += values[*child as usize] as i64;
        self.children.push(*child);
    }
}

impl Solution {
    
    // Primary testcase: [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]] \ [20,10,9,7,4,3,5] -> 40
    //               0(20)
    //            /      \
    //           2(9)     1(10)
    //          / \      /  \
    //        6(5) 5(3) 3(7) 4(4)

    fn maximum_subtree(subroot: &i32, subtrees: &HashMap<i32, TreeLevel>) -> i64 {
        if let Some(node) = subtrees.get(subroot) {
            match (node.value as i64) < node.child_sum {
                // if node value is less than the sum of its children,
                // then we are done with this branch: we "leave" the parent
                // and replace with 0 (take for our score) all childrens' subtrees
                true => Self::subtree_sum(subroot, &subtrees),
                false => node.value as i64 
                            + node.children.iter()
                                .map(|child| Self::maximum_subtree(child, &subtrees))
                                .sum::<i64>(),
            }
        } else { 0i64 }
    }
    
    fn subtree_sum(subroot: &i32, subtrees: &HashMap<i32, TreeLevel>) -> i64 {
        if let Some(node) = subtrees.get(subroot) {
            node.children.iter()
                .map(|child| Self::subtree_sum(child, &subtrees))
                .sum::<i64>()
            + node.child_sum
        } else { 0i64 }
    }
    
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        
        let mut non_leaves: HashMap<i32, TreeLevel> = HashMap::new();

        for edge in edges {
            // we cannot assume edges[i] are sorted
            // example testcase:
            // [[1,0],[2,0]] \ [7,4,3] -> 7: left edge is not the parent
            // [[0,1],[0,2]] \ [7,4,3] -> 7
            
            // Additionally, we cannot assume the lower index is the parent
            // [[0,6],[0,4],[2,6],[6,5],[4,1],[3,4]] \ [20,4,5,7,10,3,9] -> 40
            // Node indices have changed, but this is the same tree as the primary
            // testcase: we must evaluate parent by relationship from root
            //
            // ... this is stupid I should just make an adjacency list
            // I don't know why I fought it to start with :(

            let (parent, child) = match edge[0] < edge[1] {
                true => (edge[0], edge[1]),
                false => (edge[1], edge[0]),
            };
            
            if let Some(subtree) = non_leaves.get_mut(&parent) {
                subtree.add_child(&child, &values);
            } else {
                non_leaves.insert(parent, TreeLevel::new(&parent, &child, &values));
            }
        }
        
        println!("{:?}", non_leaves);
        
        Self::maximum_subtree(&0, &non_leaves)
    }
}

// Test cases
// [[0,1],[0,2],[0,3],[2,4],[4,5]]
// [5,2,5,2,1,1]
// [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]]
// [20,10,9,7,4,3,5]
// [[0,1]]
// [5,5]
// [[0,1]]
// [1,5]
// [[0,1]]
// [5,1]
// [[0,1],[0,2]]
// [7,4,3]
// [[1,0],[2,0]]
// [7,4,3]
// [[0,6],[0,4],[2,6],[6,5],[4,1],[3,4]]
// [20,4,5,7,10,3,9]
// [[4,1],[0,6],[0,4],[2,6],[6,5],[3,4]]
// [20,4,5,7,10,3,9]
// [[0,1],[0,2],[0,3]]
// [1000000000,1000000000,1000000000,1000000000]
// [[7,0],[3,1],[6,2],[4,3],[4,5],[4,6],[4,7]]
// [2,16,23,17,22,21,8,6]
