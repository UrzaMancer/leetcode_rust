use std::collections::HashMap;

/*
pub fn points_conflict_alt(first_point: (usize, usize), second_point: (usize, usize)) -> bool {
    let same_row = first_point.0 == second_point.0;
    if same_row { return true; }
    let same_column = first_point.1 == second_point.1;
    if same_column { return true; }
    let same_box = (first_point.0 / 3) == (second_point.0 / 3) && (first_point.1 / 3) == (second_point.1 / 3);
    same_box
}*/

pub fn points_conflict(point1: (usize, usize), point2: (usize, usize)) -> bool {
    (point1.0 == point2.0) || (point1.1 == point2.1) || ((point1.0 / 3) == (point2.0 / 3) && (point1.1 / 3) == (point2.1 / 3))
}

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    //let mut digit_positions: Vec<Vec<(usize, usize)>> = vec![vec![]; 10];
    let mut digit_positions: HashMap<char, Vec<(usize, usize)>> = 
        [('1', vec![]), 
        ('2', vec![]),
        ('3', vec![]),
        ('4', vec![]),
        ('5', vec![]),
        ('6', vec![]),
        ('7', vec![]),
        ('8', vec![]),
        ('9', vec![])].iter().cloned().collect();
    for (r, row) in board.iter().enumerate() {
        row.iter().enumerate()
            .filter(|(_, &char)| char != '.')
            .map(|(col, c)| {
                digit_positions.get_mut(c).unwrap().push((r, col));
                *c
            }).last();
    }
    for positions in digit_positions.values().filter(|vec| vec.len() > 1) {
        let mut unevaluated_positions = positions.clone();
        while !unevaluated_positions.is_empty() {
            let test_point = unevaluated_positions.pop().unwrap();
            for i in (0..unevaluated_positions.len()).rev() {
                if points_conflict(test_point, positions[i]) {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let test_cases: Vec<(Vec<Vec<char>>, bool)> = vec![
        (vec![
            vec!['5','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ], true),
        (vec![
            vec!['8','3','.','.','7','.','.','.','.'],
            vec!['6','.','.','1','9','5','.','.','.'],
            vec!['.','9','8','.','.','.','.','6','.'],
            vec!['8','.','.','.','6','.','.','.','3'],
            vec!['4','.','.','8','.','3','.','.','1'],
            vec!['7','.','.','.','2','.','.','.','6'],
            vec!['.','6','.','.','.','.','2','8','.'],
            vec!['.','.','.','4','1','9','.','.','5'],
            vec!['.','.','.','.','8','.','.','7','9']
        ], false),
    ];
    for (case, expected) in test_cases {
        println!("Input: {:?} |-> Output: {:?} =?= Expected: {:?}", case, is_valid_sudoku(case[..].to_vec()), expected);
    }
}
