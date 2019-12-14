use std::collections::HashMap;

/// 自写
#[test]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //利用index-value迭代模式enumerate 进行遍历
    for (i, v) in nums.iter().enumerate() {
        for (i2, v2) in nums.iter().enumerate() {
            if i == i2 { continue; }
            if v + v2 == target {
                return vec![i as i32, i2 as i32];
            }
        }
    }
    Vec::new()
}

/// 最优解
#[test]
pub fn two_sum2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut h: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        match h.get(num) {
            None => {
                h.insert(target - *num, index as i32);
            }
            Some(v) => {
                return vec![*v, index as i32];
            }
        }
    }

    vec![]
}