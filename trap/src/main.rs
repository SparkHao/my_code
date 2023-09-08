fn main() {
    println!("Hello, world!");
    let nums = vec![0,1,0,2,1,0,1,3,2,1,2,1];
    println!("{:?}", Solution::trap(nums));

    let nums = vec![4,2,0,3,2,5];
    println!("{:?}", Solution::trap(nums));
}

struct Solution {

}

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut area = 0;
        for i in 0..height.len() - 1 {
            let mut left = i;
            let mut right = height.len() - 1;
            let mut max = 0;
            while left < right {
                max = max.max(height[left].min(height[right]) * ((right - left) as i32));
                if height[left] < height[right] {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
            area += max;
        }

        area
    }
}