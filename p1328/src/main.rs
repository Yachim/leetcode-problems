struct Solution;
impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        let char_cnt = palindrome.chars().count();
        if char_cnt == 1 {return "".to_string()}

        let all_as = std::iter::repeat("a").take(char_cnt).collect::<String>();
        if palindrome == all_as {
            return format!("{}{}", palindrome[0..char_cnt - 1].to_string(), "b");
        }

        let half_len = char_cnt / 2;
        let first_half: String = palindrome[0..half_len].to_string();
        let second_half: String;
        let center_char: String;
        if char_cnt % 2 == 1 {
            center_char = palindrome.chars().nth(half_len).unwrap().to_string();
            second_half = palindrome[half_len + 1..char_cnt].to_string();
        }
        else {
            center_char = "".to_string();
            second_half = palindrome[half_len..char_cnt].to_string();
        }

        for (i, c) in first_half.chars().enumerate() {
            if c == 'a' {continue;}

            return format!("{}{}{}{}{}", first_half[0..i].to_string(), 'a', first_half[i + 1..].to_string(), center_char, second_half)
        }


        return format!("{}{}{}{}", first_half, center_char, second_half[0..half_len - 1].to_string(), 'b')
    }
}

fn main() {
    println!("{}", Solution::break_palindrome("aba".to_string()))
}
