use crate::Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {

        /* We could duplicate and re-write the entire matrix with
        cumulative sums of the columns to track the max height

        Or we can track current row with a scan function to use that
        state of the last row for reference. Can we sort in the same pass
        this way?

        [EDIT: Yes, we can]
        */
        matrix.iter()
            .scan(None, |last_row: &mut Option<Vec<i32>>, row| {
                let mut sorted: Vec<i32> = Self::vertical_accumulator_row(last_row, row);
                *last_row = Some(sorted.clone());
                Some(Self::row_max_area(sorted))
            })
            .max()
            .unwrap()
    }

    fn vertical_accumulator_row(last_row: &mut Option<Vec<i32>>, row: &Vec<i32>) -> Vec<i32> {
        match last_row {
            Some(lrow) => 
                row.iter().zip(lrow.iter())
                    .map(|(&num, &above)| match num {
                        0 => 0,
                        _ => num + above,
                    }).collect(),
            None => row.to_vec(),
        }
    }

    fn row_max_area(mut sorted: Vec<i32>) -> i32 {
        sorted.sort_unstable();
        sorted.iter().rev().enumerate()
            .map(|(left_base, &height)| height * (left_base as i32 + 1))
            .max()
            .unwrap()
    }
}
