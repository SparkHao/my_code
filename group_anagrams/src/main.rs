fn main() {
    println!("group_anagrams!");
    let strs: Vec<String> = vec!["eat", "tea", "tan", "ate", "nat", "bat"].iter().map(|s| s.to_string()).collect();
    println!("{:?}", Solution::group_anagrams(strs));
}

struct Solution {

}

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        if strs.len() == 0 {
            return vec![];
        }
        let mut map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            let key = chars.iter().collect::<String>();
            if map.contains_key(&key) {
                map.get_mut(&key).unwrap().push(s);
            } else {
                map.insert(key, vec![s]);
            }
        }
        map.values().map(|v| v.to_vec()).collect()
    }
}