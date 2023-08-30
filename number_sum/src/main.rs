fn main() {
    println!("number sum");
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    // let nums = vec![3, 2, 4];
    // let target = 6;
    // let nums = vec![3, 3];
    // let target = 6;
    println!("{:?}", Solution::two_sum(nums, target));
}

struct Solution {

}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = std::collections::HashMap::new();
        nums.into_iter()
            .enumerate()
            .find_map(|(i, x)| match map.get(&(target - x)) {
                Some(j) => Some(vec![i as i32, *j]),
                None => {
                    map.insert(x, i as i32);
                    None
                }
            }).unwrap()
    }
}