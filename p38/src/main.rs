struct Solution;
impl Solution {
    pub fn say(n: String) -> String {
        let first = n.chars().nth(0).unwrap();
        let rem = n[1..].chars();

        let mut out = "".to_string();
        let mut last = first;
        let mut cnt = 1;
        for c in rem {
            if c != last {
                out += &format!("{}{}", cnt, last).to_string();
                last = c;
                cnt = 1;
                continue;
            }
            
            cnt += 1;
        }

        out += &format!("{}{}", cnt, last).to_string();

        return out;
    }

    pub fn count_and_say(n: i32) -> String {
        if n == 1 {return "1".to_string();}

        return Solution::say(
            Solution::count_and_say(
                n - 1
            )
        );
    }
}

fn main() {
    println!("{}", Solution::count_and_say(4));
}
