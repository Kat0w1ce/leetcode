fn main() {}
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut diff = nums[0] + nums[1] + nums[2] - target;
    let mut rst = diff + target;
    diff = i32::abs(diff);
    for i in 0..nums.len() {
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k && j < nums.len() && k > 0 {
            let sum = nums[i] + nums[j] + nums[k];
            let newdiff = i32::abs(sum - target);
            if newdiff == 0 {
                return 0;
            }
            if newdiff < diff {
                diff = newdiff;
                rst = sum;
            }
            if sum > target {
                k -= 1;
            } else {
                j += 1;
            }
        }
    }

    rst
}
