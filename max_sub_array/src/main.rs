fn main() {
    println!("Hello, world!");
    let nums = vec![-2,1,-3,4,-1,2,1,-5,4];
    println!("{:?}", Solution::max_sub_array(nums));

    let nums = vec![1];
    println!("{:?}", Solution::max_sub_array(nums));

    let nums = vec![5,4,-1,7,8];
    println!("{:?}", Solution::max_sub_array(nums));

    let nums = vec![-1];
    println!("{:?}", Solution::max_sub_array(nums));
}

struct Solution {}

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut pre = 0;
        for n in nums {
            pre = (pre + n).max(n);
            max = max.max(pre);
        }
        max
    }
}