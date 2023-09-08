fn main() {
    println!("Hello, world!");
    let mut matrix = vec![vec![1,1,1],vec![1,0,1],vec![1,1,1]];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);

    let mut matrix = vec![vec![0,1,2,0],vec![3,4,5,2],vec![1,3,1,5]];
    Solution::set_zeroes(&mut matrix);
    println!("{:?}", matrix);
}

struct Solution {}

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let mut col = vec![];
        let mut row = vec![];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if matrix[i][j] == 0 {
                    col.push(j);
                    row.push(i);
                }
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                if col.contains(&j) || row.contains(&i){
                    matrix[i][j] = 0;
                }
            }
        }
    }
}