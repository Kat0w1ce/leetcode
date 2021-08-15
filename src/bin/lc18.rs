fn main() {
    four_sum(vec![1, 0, -1, 0, -2, 2], 0);
    four_sum(vec![2, 2, 2, 2, 2, 2], 8);
    four_sum(vec![-2, -1, -1, 1, 1, 2, 2], 0);
}
pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut rst = vec![];
    if nums.len() < 4 {
        return rst;
    }
    let mut nums = nums;
    nums.sort();
    for i in 0..nums.len() - 3 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        for j in i + 1..nums.len() - 2 {
            if j > i + 1 && nums[j] == nums[j - 1] {
                // println!("d");
                continue;
            }
            let mut left = j + 1;
            let mut right = nums.len() - 1;
            let two_sum = nums[i] + nums[j];
            let delta = target - two_sum;
            while left < right && left > j && right < nums.len() {
                match delta.cmp(&(nums[left] + nums[right])) {
                    std::cmp::Ordering::Greater => left += 1,
                    std::cmp::Ordering::Equal => {
                        rst.push(vec![nums[i], nums[j], nums[left], nums[right]]);
                        // println!("{:?}", rst);
                        right -= 1;
                        while right > left && nums[right] == nums[right + 1] {
                            // println!("a");
                            right -= 1;
                        }
                        left += 1;
                        while left < right && nums[left] == nums[left - 1] {
                            // println!("b");
                            left += 1;
                        }
                    }
                    _ => right -= 1,
                }
            }
        }
    }
    rst
}
