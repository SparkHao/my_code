use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
    // let nums = vec![100, 4, 200, 1, 3, 2];
    // let nums = vec![9,1,4,7,3,-1,0,5,8,-1,6];
    let nums = vec![0,3,7,2,-1,8,4,6,0,1];
    println!("{:?}", Solution::longest_consecutive(nums));
}

struct Solution {

}

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // if nums.len() == 0 {
        //     return 0;
        // }   
        // let mut nums = nums;
        // nums.sort();
        // let mut res = Vec::new();
        // let mut flag = false;
        // let mut max = 0;
        // for i in 1..nums.len() {
        //     if nums[i] == nums[i-1] {
        //     }else {
        //         if nums[i] - nums[i-1] == 1 {
        //             if flag == true {
        //                 if max < res.len() {
        //                     max = res.len();
        //                 }
        //                 flag = false;
        //                 res.clear();
        //             }
        //             res.push(nums[i]);
        //         }else {
        //             flag = true;
        //         }
        //     }
        // }
        // println!("{:?}, max: {}", res, max);
        // let a = res.len().max(max);
        // a as i32 + 1

        // ---------------------------------

        let mut d: HashSet<i32> = nums.iter().map(|&i| i).collect();
        println!("d: {:?}", d);
        fn f(i: i32, d: &mut HashSet<i32>) -> i32 {
            if d.contains(&i) {
                d.remove(&i);
                return 1 + f(i + 1, d) + f(i - 1, d)
            }   
            0
        }
        nums.into_iter().map(|i| f(i, &mut d)).max().unwrap_or(0)
    }
}