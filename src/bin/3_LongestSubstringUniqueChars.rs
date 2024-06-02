// 3. Longest Substring Without Repeating Characters
// Solution: https://leetcode.com/submissions/detail/1275756175/

use std::{cmp, collections::HashMap};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut map: HashMap<char, usize> = HashMap::new();

    let mut max = 0;
    let mut l = 0;

    for (i, c) in s.chars().enumerate() {
        l = match map.get(&c) {
            Some(&n) => cmp::max(n + 1, l),
            _ => l,
        };

        map.insert(c, i);

        max = cmp::max(max, (i - l) + 1);
    }

    max as i32
}

pub fn main() {
    let result = length_of_longest_substring(String::from("bbbb"));
    println!("The result is {result}")
}
