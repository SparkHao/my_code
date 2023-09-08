fn main() {
    println!("Hello, world!");
    let s = String::from("abcabcbb");
    println!("{:?}", Solution::length_of_longest_substring(s));

    let s = String::from("bbbbb");
    println!("{:?}", Solution::length_of_longest_substring(s));

    let s = String::from("pwwkew");
    println!("{:?}", Solution::length_of_longest_substring(s));

    let s = String::from("aa");
    println!("{:?}", Solution::length_of_longest_substring(s));

    let s = String::from("au");
    println!("{:?}", Solution::length_of_longest_substring(s));
}

struct Solution {

}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.len() < 2 {
            return s.len() as i32;
        }
        
        let mut count = 0;
        for i in 0..s.len() - 1 {
            let mut res = Vec::new();
            let mut p = i;
            let array: Vec<char> = s.chars().collect();
            while p < s.len()  {
                if !res.contains(&array[p]) {
                    res.push(array[p]);
                }else {
                    break;
                }
                p += 1;
            }

            count = count.max(res.len() as i32);
        }   

        count
    }
}