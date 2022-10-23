struct Solution;
impl Solution {
    pub fn find_error_nums(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort();

        let len = nums.len() as i32;
        let target_s: i32 = (1 + len) * len / 2;

        let s: i32 = nums.iter().sum();
        nums.dedup();
        let uniq_s: i32 = nums.iter().sum();

        return vec![s - uniq_s, target_s - uniq_s];            
    }
}

fn main() {
    println!("{:?}", Solution::find_error_nums(vec![3,2,2]));
}
