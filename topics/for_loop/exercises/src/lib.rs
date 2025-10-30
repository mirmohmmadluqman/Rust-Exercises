pub fn sum(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut i = 0;
    while i < nums.len() {
        total += nums[i];
        i += 1;
    }
    total
}


pub fn fill(i: u32, n: usize) -> Vec<u32> {
    let mut result = Vec::new();
    let mut count = 0;
    while count < n {
        result.push(i);
        count += 1;
    }
    result
}
