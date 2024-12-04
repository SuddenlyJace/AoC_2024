use regex::Regex;

pub fn aoc(input: String) -> (i32, i32) {
    let data = input.replace("do()", "mul(1,1)").replace("don't()", "mul(0,0)");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    let mut sum_modified = 0;
    let mut enabled = true;
    for (_, [a, b]) in re.captures_iter(&data).map(|c| c.extract()) {
        let int_a = a.parse::<i32>().unwrap();
        let int_b = b.parse::<i32>().unwrap();
        if int_a == 1 && int_b == 1 {
            enabled = true;
        } else if int_a == 0 && int_b == 0 {
            enabled = false;
        } else {
            if enabled {
                sum_modified += int_a * int_b;
            }
            sum += int_a * int_b;
        }
        
    }

    (sum, sum_modified)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE_2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    
    #[test]
    fn test_example() {
        assert_eq!(aoc(EXAMPLE.to_string()), (161, 161))
    }

    #[test]
    fn test_example2() {
        assert_eq!(aoc(EXAMPLE_2.to_string()), (161, 48))
    }
}