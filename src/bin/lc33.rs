fn main() {}

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    if nums.len() == 1 && nums[0] == target {
        return 0;
    }
    if nums.len() == 1 && nums[0] != target {
        return -1;
    }

    let idx = nums
        .binary_search_by(|x| {
            if *x > nums[0] {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        })
        .unwrap_or_else(|i| i);
    println!("idx=={}", idx);
    if let Ok(index) = &nums[..idx + 1].binary_search(&target) {
        return *index as i32;
    }
    if let Ok(index) = &nums[idx..].binary_search(&target) {
        return (*index + idx) as i32;
    }
    -1
}

#[cfg(test)]
mod tests {
    use crate::search;

    #[test]
    fn test_search() {
        assert_eq!(search(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(search(vec![3, 1], 3), 0)
    }
}
