fn main() {
    println!(
        "{:?}",
        three_sum(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6])
    )
}
pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums;
    let mut rst: Vec<Vec<i32>> = vec![];
    nums.sort();
    if nums.len() < 3 || nums[nums.len() - 1] < 0 || nums[0] > 0 {
        return vec![];
    }
    for i in 0..nums.len() {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let two_sum = -nums[i];
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < nums.len() && k > 0 && j < k {
            match (nums[j] + nums[k]).cmp(&two_sum) {
                std::cmp::Ordering::Greater => k -= 1,
                std::cmp::Ordering::Equal => {
                    rst.push(vec![nums[i], nums[j], nums[k]]);
                    k -= 1;
                    while j < k && nums[k] == nums[k + 1] {
                        k -= 1;
                    }
                    j += 1;
                    while j < k && nums[j] == nums[j - 1] {
                        j += 1;
                    }
                    // no duplicated result
                }
                _ => j += 1,
            }
        }
    }
    rst
}

#[cfg(test)]
mod tests {
    use crate::three_sum;

    #[test]
    fn test_three_sum() {
        assert_eq!(
            three_sum(vec![-1, 0, 1, 2, -1, -4]),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        )
    }
}
