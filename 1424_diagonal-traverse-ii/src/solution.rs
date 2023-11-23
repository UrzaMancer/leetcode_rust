use crate::Solution;

use std::collections::VecDeque;

struct Location {
    x: usize,
    y: usize,
}

impl Location {
    pub fn below(&self) -> Self {
        Self { y: self.y + 1, x: self.x }
    }
    pub fn right(&self) -> Self {
        Self { x: self.x + 1, y: self.y }
    }
    pub fn origin() -> Self {
        Self { x: 0, y: 0, }
    }
    pub fn eq(&self, other: &Location) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Cell {
    position: Location,
    value: i32,
}

impl Cell {
    pub fn add_cell(nums: &Vec<Vec<i32>>, pos: Location) -> Option<Self> {
        if let Some(row) = nums.get(pos.y) {
            if let Some(&element) = row.get(pos.x) {
                Some(Self { position: pos, value: element })
            } else { None }
        } else { None }
    }

    pub fn same(&self, other: &Cell) -> bool {
        self.position.eq(&other.position)
    }

    pub fn queue_neighbors(&self, nums: &Vec<Vec<i32>>, traverse_path: &mut VecDeque<Cell>) {
        match (traverse_path.back(), Cell::add_cell(&nums, self.position.below())) {
            (Some(last), Some(bot)) => if !bot.same(last) { traverse_path.push_back(bot); },
            (None, Some(bot)) => traverse_path.push_back(bot),
            _ => (),
        };
        match Cell::add_cell(&nums, self.position.right()) {
            Some(right) => traverse_path.push_back(right),
            _ => (),
        };
        /*
        if let Some(bot_cell) = Cell::add_cell(&nums, self.position.below()) {
            if let Some(last_added) = traverse_path.back() {
                if last_added.position.x != bot_cell.position.x || last_added.position.y != bot_cell.position.y { traverse_path.push_back(bot_cell); }
            } else { traverse_path.push_back(bot_cell); }
        }
        if let Some(right_cell) = Cell::add_cell(&nums, self.position.right()) {
            traverse_path.push_back(right_cell);
        }
        */
    }
}

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let mut traverse_path: VecDeque<Cell> =
            match Cell::add_cell(&nums, Location::origin()) {
                None => { return vec![]; },
                Some(c) => VecDeque::from([c]),
            };

        (1..).scan(&mut traverse_path, |queue, _|
            match queue.pop_front() {
                Some(next) => { 
                    next.queue_neighbors(&nums, queue); 
                    Some(next.value) 
                    },
                None => None,
            }).collect()

    }
}

/*

Credit to lopnao: https://leetcode.com/lopnao/ for helping verify I was on the right track:
https://leetcode.com/problems/diagonal-traverse-ii/solutions/4316828/solution-with-vecdeque/

impl Solution {
    pub fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
        let max_len = nums.iter().map(|row| row.len()).max().unwrap_or(0);

        let mut ans: Vec<VecDeque<i32>> = vec![VecDeque::new(); nums.len() + max_len];
        for (ind, arr) in nums.iter().enumerate() {
            for (i, d) in arr.iter().enumerate() {
                ans[i+ind].push_front(*d);
            }
        }

        ans.into_iter().flatten().collect()

    }
}
*/
