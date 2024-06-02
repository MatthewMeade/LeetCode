// 1. Two Sum
// Submission https://leetcode.com/submissions/detail/1262689453/

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();

    for i in 0..(nums.len()) {
        if m.contains_key(&nums[i]) {
            return vec![i as i32, m.get(&nums[i]).unwrap().clone() as i32];
        }

        m.insert(target - nums[i], i);
    }

    return vec![];
}

pub fn main() {
    let result = two_sum(vec![1, 2, 3], 3);
    println!("The result is: {:?}", result)
}
