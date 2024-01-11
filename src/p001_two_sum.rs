use std::collections::HashMap;
#[allow(unused)]
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut mp = HashMap::new();
        let mut result = Vec::new();
        for (first, &val) in nums.iter().enumerate() {
            let d = target - val;
            if let Some(&second) = mp.get(&d) {
                result.push(first as i32);
                result.push(second as i32);
            };

            if result.len() == 2 {
                break;
            }

            mp.insert(val, first);
        }
        result
    }
}
