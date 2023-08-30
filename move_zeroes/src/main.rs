fn main() {
    println!("Hello, world!");
    let mut nums = vec![0, 1, 0, 3, 12];
    // let mut nums = vec![0];
    // let mut nums = vec![2, 1];
    // let mut nums = vec![0, 0, 1];
    Solution::move_zeroes(&mut nums);
    println!("{:?}", nums);
}

struct Solution {

}

impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // let len = nums.len();
        // if len == 0 || len == 1 {
            
        // }else {
        //     for i in 0..len-1 {
        //         if nums[i] == 0 {
        //             for j in i+1..len {
        //                 if nums[j] != 0 {
        //                     nums.swap(i, j);
        //                     break;
        //                 }
        //             }
        //         }
        //         println!("i: {}, nums: {:?}", i, nums);
        //     }
        // }

        let mut j = 0;
        for i in 0..nums.len() {
            if nums[i] != 0 {
                nums[j] = nums[i];
                if i != j {
                    nums[i] = 0;
                }
                j+=1;
            }
        }

    }
}