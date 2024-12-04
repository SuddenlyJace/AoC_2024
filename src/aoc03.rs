use regex::Regex;

pub fn aoc(input: String) -> (i32, i32) {
    // WE NEED A VECTOR OF VECTORS TO CONTAIN ALL THE REPORTS!!!
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for (_, [a, b]) in re.captures_iter(&input).map(|c| c.extract()) {
        sum += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap();
    }

    (sum, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    #[test]
    fn test_example() {
        assert_eq!(aoc(EXAMPLE.to_string()), (161, 0))
    }
}