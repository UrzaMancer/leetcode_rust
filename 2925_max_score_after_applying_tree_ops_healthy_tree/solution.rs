use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct TreeLevel {
    value: i32,
    adjacencies: Vec<i32>,
}

impl TreeLevel {
    pub fn new(val: i32) -> Self {
        Self {
            value: val,
            adjacencies: Vec::new(),
        }
    }
    
    pub fn add_adjacent(&mut self, relative: i32) {
        self.adjacencies.push(relative);
    }
}

impl Solution {
    
    // Primary testcase: [[0,1],[0,2],[1,3],[1,4],[2,5],[2,6]] \ [20,10,9,7,4,3,5] -> 40
    //               0(20)
    //            /      \
    //           2(9)     1(10)
    //          / \      /  \
    //        6(5) 5(3) 3(7) 4(4)

    fn maximum_subtree(subroot: &i32, parent: &i32, subtrees: &HashMap<i32, TreeLevel>) -> i64 {
        if let Some(node) = subtrees.get(subroot) {
            if subroot != parent && node.adjacencies.len() <= 1 { return 0i64; }
            let c_sum: i64 = 
                        node.adjacencies.iter()
                            .map(|adj| match (adj == parent, subtrees.get(adj)) {
                                    (false, Some(child)) => child.value as i64,
                                    _ => 0i64
                            })
                            .sum::<i64>();

            match (node.value as i64) < c_sum {
                // if node value is less than the sum of its children,
                // then we are done with this branch: we "leave" the parent
                // and replace with 0 (take for our score) all childrens' subtrees
                true => Self::subtree_sum(subroot, parent, &subtrees) - node.value as i64,
                false => node.value as i64 
                            + node.adjacencies.iter()
                                .map(|adj| match adj == parent {
                                    true => 0i64,
                                    false => Self::maximum_subtree(adj, parent, &subtrees),
                                })
                                .sum::<i64>(),
            }
        } else { 0i64 }
    }
    
    fn subtree_sum(subroot: &i32, parent: &i32, subtrees: &HashMap<i32, TreeLevel>) -> i64 {
        if let Some(node) = subtrees.get(subroot) {
            node.adjacencies.iter()
                .map(|adj| match parent == adj {
                    false => Self::subtree_sum(adj, subroot, &subtrees),
                    true => 0i64,
                    })
                .sum::<i64>()
            + node.value as i64
        } else { 0i64 }
    }
    
    pub fn maximum_score_after_operations(edges: Vec<Vec<i32>>, values: Vec<i32>) -> i64 {
        
        let mut nodes: HashMap<i32, TreeLevel> = HashMap::with_capacity(values.len());

        for edge in edges {
            // we cannot assume edges[i] are sorted
            // example testcase:
            // [[1,0],[2,0]] \ [7,4,3] -> 7: left edge is not the parent
            // [[0,1],[0,2]] \ [7,4,3] -> 7
            
            // Additionally, we cannot assume the lower index is the parent
            // [[0,6],[0,4],[2,6],[6,5],[4,1],[3,4]] \ [20,4,5,7,10,3,9] -> 40
            // Node indices have changed, but this is the same tree as the primary
            // testcase: we must evaluate parent by relationship from root

            let (u, v) = (edge[0], edge[1]);
            nodes.entry(u).or_insert(TreeLevel::new(values[u as usize]))
                .add_adjacent(v);
            nodes.entry(v).or_insert(TreeLevel::new(values[v as usize]))
                .add_adjacent(u);   
        }
        
        //println!("{:?}", nodes);
        
        Self::maximum_subtree(&0, &0, &nodes)
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
// [[1,0],[9,1],[6,2],[7,4],[3,5],[7,3],[9,6],[7,8],[7,9]]
// [14,17,13,18,17,10,23,19,22,2] -> 153
