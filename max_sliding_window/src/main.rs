fn main() {
    println!("Hello, world!");
    let nums = vec![1,3,-1,-3,5,3,6,7];
    let k = 3;
    println!("{:?}", Solution::max_sliding_window(nums, k));

    let nums = vec![1];
    let k = 1;
    println!("{:?}", Solution::max_sliding_window(nums, k));
}

struct Solution {}

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        // let mut res = vec![];
        // for i in 0..(nums.len() - k as usize + 1) {
        //     res.push(*nums[i..i+k as usize].iter().max().unwrap());
        // }
        // res
        let mut res = vec![];
        let mut deq = std::collections::VecDeque::new();
        for i in 0..nums.len() {
            // 保证从大到小 如果前面数小则需要依次弹出，直至满足要求
            while !deq.is_empty() && nums[*deq.back().unwrap()] <= nums[i] {
                deq.pop_back();
            }
            deq.push_back(i); // 添加当前值对应的数组下标
            if i == deq[0] + k as usize { // 判断当前队列中队首的值是否有效
                deq.pop_front();
            }
            if i >= k as usize - 1 { // 当窗口长度为k时 保存当前窗口中最大值
                res.push(nums[deq[0]]);
            }
        }
        res
    }
}