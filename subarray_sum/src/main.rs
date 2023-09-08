fn main() {
    println!("Hello, world!");
    let nums = vec![1,1,1]; 
    let k = 2;
    println!("{:?}", Solution::subarray_sum(nums, k));

    let nums = vec![1, 2, 3]; 
    let k = 3;
    println!("{:?}", Solution::subarray_sum(nums, k));

    let nums = vec![-1,-1,1];
    let k = 0;
    println!("{:?}", Solution::subarray_sum(nums, k));

    let nums = vec![1,-1,0];
    let k = 0;
    println!("{:?}", Solution::subarray_sum(nums, k));

    
}
struct Solution {}

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = 0;
        for i in 0..nums.len() {
            let mut end = i as i32;
            let mut sum = 0;
            loop {
                if end < 0 {
                    break;
                }
                sum += nums[end as usize];
                if sum == k {
                    count += 1;
                }
                end -= 1;
            }
        }
        count
    }
}