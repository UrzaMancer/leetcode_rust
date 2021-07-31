use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();
    for count in 10..110 {
        let mut matrix: Vec<Vec<u8>> = Vec::new();
        let mut target: Vec<Vec<u8>> = Vec::new();
        let n = count / 10;
        while matrix.len() < n {
            let mut mat_row: Vec<u8> = Vec::new();
            let mut targ_row: Vec<u8> = Vec::new();
            while mat_row.len() < n {
                mat_row.push(rng.gen_range(0, 2));
                targ_row.push(rng.gen_range(0, 2));
            }
            matrix.push(mat_row);
            target.push(targ_row);
        }
        println!("{:?}", matrix);
        println!("{:?}", target);
    }

}