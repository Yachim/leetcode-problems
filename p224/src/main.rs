struct Solution {}
impl Solution {
    fn convert_to_postfix(s: String) -> Vec<String> {
        let mut out_queue: Vec<String> = vec![];
        let mut op_stack: Vec<String> = vec![];

        let ops = vec!['+', '-'];
        let mut num = "".to_string();
        for c in s.chars() {
            if c.is_digit(10) {
                num += &c.to_string();
            }
            else if num != "" {
                out_queue.push(num.to_string());
                num = "".to_string();
            }
            
            if ops.contains(&c) {
                while op_stack.len() > 0 && op_stack[op_stack.len() - 1] != "(" {
                    out_queue.push(op_stack.pop().unwrap());
                }
                op_stack.push(c.to_string());
            }
            else if c == '(' {
                op_stack.push(c.to_string());
            }
            else if c == ')' {
                while op_stack[op_stack.len() - 1] != "(" {
                    out_queue.push(op_stack.pop().unwrap());
                }
                op_stack.pop();
            }
        }
        if num != "" {
            out_queue.push(num.to_string());
        }

        op_stack.reverse();
        out_queue = [out_queue, op_stack].concat(); 

        return out_queue;
    }

    fn solve_postfix(p: Vec<String>) -> i32 {
        let mut stack: Vec<i32> = vec![];
        for token in p {
            if token.len() > 1 || token.chars().nth(0).unwrap().is_digit(10) {
                stack.push(token.parse().unwrap());
            }
            else {
                let tok_1 = stack.pop().unwrap();
                let tok_2 = stack.pop().unwrap();
                if token == "+" {
                    stack.push(tok_2 + tok_1);
                }
                else {
                    stack.push(tok_2 - tok_1);
                }
            }
        }

        return stack[0];
    }

    pub fn calculate(mut s: String) -> i32 {
        s = s.replace(" ", "");
        if !(s.contains('+') || s.contains('-')) {
            s = s.replace("(", "").replace(")", "");
            return s.parse().unwrap();
        }

        if s.chars().nth(0).unwrap() == '-' {
            s = format!("0{}", s);
        }
        s = s.replace("(-", "(0-");

        let postfix = Solution::convert_to_postfix(s);
        return Solution::solve_postfix(postfix);
    }
}

fn main() {
    println!("{}", Solution::calculate("(1+(4+5+2)-3)+(6+8)".to_string()));
}
