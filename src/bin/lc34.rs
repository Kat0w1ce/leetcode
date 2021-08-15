fn main() {
    assert_eq!(search_range(vec![5, 7, 7, 8, 8, 10], 8), vec![3, 4]);
    assert_eq!(search_range(vec![8, 9], 8), vec![0, 0]);
    assert_eq!(search_range(vec![2, 2], 2), vec![0, 1]);
}

pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut rst = vec![];
    if nums.len() == 1 && nums[0] == target {
        return vec![0, 0];
    }
    if nums.is_empty() {
        return vec![-1, -1];
    }
    match nums.binary_search(&target) {
        Ok(idx) => {
            let mut left = idx;
            let mut right = idx;
            for _ in (0..idx).rev() {
                if nums[left] != nums[idx] {
                    rst.push((left + 1) as i32);
                    break;
                } else {
                    left -= 1;
                }
            }
            if rst.len() == 0 {
                rst.push(left as i32);
            }
            for _ in idx..nums.len() {
                if nums[right] != nums[idx] {
                    rst.push((right - 1) as i32);
                    break;
                } else {
                    right += 1;
                }
            }
            if rst.len() == 1 {
                rst.push(right as i32);
            }
        }
        Err(_) => return vec![-1, -1],
    }
    rst
}
