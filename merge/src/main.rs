fn main() {
    println!("Hello, world!");
    let intervals = vec![vec![1,3],vec![2,6],vec![8,10],vec![15,18]];
    println!("{:?}", Solution::merge(intervals));
    let intervals = vec![vec![1,4],vec![4,5]];
    println!("{:?}", Solution::merge(intervals));
    let intervals = vec![vec![1,4],vec![5,6]];
    println!("{:?}", Solution::merge(intervals));

    let intervals = vec![vec![1,4],vec![2,3]];
    println!("{:?}", Solution::merge(intervals));
}


struct Solution{}

impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.len() == 1 {
            return intervals;
        }
        let mut intervals = intervals.clone();
        intervals.sort_by(|a, b| a[0].partial_cmp(&b[0]).unwrap());
        let mut res: Vec<Vec<i32>> = Vec::new();
        for i in 0..intervals.len(){
            let left = intervals[i][0];
            let right = intervals[i][1];
            let l = res.len();
            if l == 0 || res[l - 1][1] < left {
                res.push(vec![left, right]);
            }else {
                res[l - 1][1] = res[l - 1][1].max(right);
            }
        }
        res
    }
}