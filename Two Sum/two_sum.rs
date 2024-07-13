use std::collections::HashMap;

fn two_sum_brute_force(nums: &[i32], target: i32) -> Vec<usize> {
    for (i, &value_i) in nums.iter().enumerate() {
        for (j, &value_j) in nums
            .iter()
            .enumerate()
            .skip(i + 1) {
            if value_i + value_j == target {
                return vec![i, j];
            }
        }
    }

    vec![]
}

fn two_sum_optimised(nums: &[i32], target: i32) -> Vec<i32> {
    let mut hashmap: HashMap<i32, i32> = HashMap::new();

    for (i, &current) in nums.iter().enumerate() {
        let diff: i32 = target - current;

        if let Some(&value) = hashmap.get(&diff) {
            return vec![value, i as i32];
        }

        hashmap.insert(current, i as i32);
    }

    vec![]
}

fn main() {
    let nums: Vec<i32> = vec![2, 7, 9, 11, 13];
    let target: i32 = 9;

    let result_brute_force = two_sum_brute_force(&nums, target);
    println!("Brute force result: {:?}", result_brute_force);

    let result_optimsed = two_sum_optimised(&nums, target);
    println!("Optimised result: {:?}", result_optimsed)
}
