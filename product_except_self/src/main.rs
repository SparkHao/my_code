fn main() {
    println!("Hello, world!");
    let nums = vec![1,2,3,4];
    println!("{:?}", Solution::product_except_self(nums));

    let nums = vec![-1,1,0,-3,3];
    println!("{:?}", Solution::product_except_self(nums));
}

struct Solution{}

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        let mut total = 1;
        nums.iter().for_each(|x| {
            total *= x;
        });
        for i in 0..nums.len() {
            if nums[i] == 0 {
                let mut tmp = 1;
                for j in 0..nums.len() {
                    if i != j {
                        tmp *= nums[j];
                    }
                }
                res.push(tmp);
            }else {
                res.push(total / nums[i]);
            }
            
        }
        res
    }
}