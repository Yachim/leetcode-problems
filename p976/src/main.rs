struct Solution {}
impl Solution {
    pub fn largest_perimeter(mut nums: Vec<i32>) -> i32 {
        nums.sort_by(|a, b| b.cmp(a));    
        
        while nums.len() >= 3 {
            if nums[1] + nums[2] <= nums[0] {
                nums.remove(0);
                continue;
            }

            return nums[0] + nums[1] + nums[2];
        }

        return 0;
    }
}

fn main() {
    println!("{}", Solution::largest_perimeter(vec![3,6,2,3]));
}
