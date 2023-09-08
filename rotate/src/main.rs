fn main() {
    println!("Hello, world!");
    let mut nums = vec![1,2,3,4,5,6,7];
    let k = 3;
    Solution::rotate(&mut nums, k);
    println!("{:?}", nums);

    let mut nums = vec![-1,-100,3,99];
    let k = 2;
    Solution::rotate(&mut nums, k);
    println!("{:?}", nums);
}

struct Solution{}

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        let k = k as usize;
        let mut mid = vec![0; l];
        for i in 0..nums.len() {
            mid[(i + k) % l] = nums[i];
        }
        nums.copy_from_slice(&mid);
    }

    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        for _ in 0..(k as usize % nums.len()) {
            let n = nums.pop().unwrap();
            nums.insert(0, n);
        }
    }

    pub fn rotate3(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        let n = nums.len();
        nums.reverse();
        nums[0..k].reverse();
        nums[k..n].reverse();
    }

    pub fn rotate4(nums: &mut Vec<i32>, k: i32) {
        let k = k as usize % nums.len();
        nums.rotate_right(k);
    }
}