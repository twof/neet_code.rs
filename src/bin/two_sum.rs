pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    let mut nums: Vec<_> = nums.iter().enumerate().collect();
    nums.sort_by(|l, r| l.1.cmp(r.1));

    while left < right {
        let left_val = nums[left];
        let right_val = nums[right];
        let sum = left_val.1 + right_val.1;

        if sum == target {
            return vec![left_val.0 as i32, right_val.0 as i32];
        } else if sum > target {
            right -= 1;
        } else {
            left += 1;
        }
    };

    return vec![-1, -1];
}

fn main() {
    let result = two_sum(vec![3,2,4], 6);

    println!("{result:?}")
}