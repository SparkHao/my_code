fn main() {
    println!("Hello, world!");

    let nums = vec![-1,0,1,2,-1,-4];
    println!("{:?}", Solution::three_sum(nums));
    
    let nums = vec![0,1,1];
    println!("{:?}", Solution::three_sum(nums));

    let nums = vec![0,0,0];
    println!("{:?}", Solution::three_sum(nums));

    let nums = vec![0,-4,-1,-4,-2,-3,2];
    println!("{:?}", Solution::three_sum(nums));
}

struct Solution {

}

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums.clone();
        nums.sort();
        let mut res = Vec::new();
        let mut map = std::collections::HashMap::new();
        for i in 0..nums.len() - 1 {
            let mut left = i + 1;
            let mut right = nums.len() - 1;
            
            while left < right {
                let sum = nums[left] + nums[i] + nums[right];
                if sum == 0 {
                    let temp = vec![nums[i], nums[left], nums[right]];
                    if !map.contains_key(&temp) {
                        res.push(temp.clone());
                        map.insert(temp, 1);
                        while left < right && nums[left] == nums[left + 1] {
                            left += 1;
                        }
                        while left < right && nums[right] == nums[right - 1] {
                            right -= 1;
                        }
                    }
                    
                    left += 1;
                }else if sum > 0 {
                    right -= 1;
                }else {
                    left += 1;
                }

            }
        }
        res
    }
}