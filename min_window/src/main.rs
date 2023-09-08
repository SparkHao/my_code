fn main() {
    println!("Hello, world!");
    let s = String::from("ADOBECODEBANC");
    let t = String::from("ABC");
    println!("{:?}", Solution::min_window(s, t));

    let s = String::from("a");
    let t = String::from("a");
    println!("{:?}", Solution::min_window(s, t));

    let s = String::from("a");
    let t = String::from("aa");
    println!("{:?}", Solution::min_window(s, t));


}

struct Solution {}

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        // let mut v1 = vec![0; 26];
        // let mut v2 = vec![0; 26];

        // t.as_bytes().iter().map(|v| {
        //     v1[(*v - 97) as usize] += 1;
        // });

        // s.as_bytes().iter().map(|v| {
        //     v2[(*v - 97) as usize] += 1;
        // });
        String::new()
    }
}