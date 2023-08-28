use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let set: HashSet<i32> = nums.iter().cloned().collect();
    set.len() != nums.len()
}

fn main() {
    let is_dupe = contains_duplicate(vec![1, 1, 2, 3, 4]);
    print!("{is_dupe}");
}