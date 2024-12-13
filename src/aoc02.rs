use std::iter::Peekable;

pub fn aoc(input: String) -> (i32, i32) {
    // WE NEED A VECTOR OF VECTORS TO CONTAIN ALL THE REPORTS!!!
    let mut reports: Vec<Vec<u32>> = Vec::new();
    
    // Populate our vectors with our data
    // PUSH that report into our reports
    let lines = input.lines();
    for line in lines {
        let report: Vec<u32> = line
            .split(" ")
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();
        reports.push(report);
    }

    // Time to measure some levels
    let mut safe_reports: i32 = 0;
    let mut dampened_reports: i32 = 0;

    for report in reports {
        // Setup our loop
        let report_iter = report.iter().peekable();
        let mut reverse_report = report.clone();
        reverse_report.reverse();
        let report_reverse_iter = reverse_report.iter().peekable();

        let (mut safe, mut dampen_triggered) = check_report(report_iter);

        if safe == false {
            (safe, dampen_triggered) = check_report(report_reverse_iter);
        }
        println!("Safe? {}", safe);

        if safe && (dampen_triggered == false) {
            safe_reports += 1;
            dampened_reports += 1;
        } else if safe {
            dampened_reports += 1;
        }
        println!("{:?}", report);
    }

    (safe_reports, dampened_reports)
}

fn check_report(mut report_iter: Peekable<std::slice::Iter<'_, u32>>) -> (bool, bool) {
    let mut safe: bool = true;
    let mut dampen_triggered: bool = false;

    let mut prev_level = report_iter.next().unwrap();
    let level = *(report_iter.peek().unwrap());
    let increasing = is_increasing(prev_level, level);

    // Loop over each level in report and compare against previous
    for level in report_iter {
        if increasing != is_increasing(prev_level, &level) {
            safe = false
        }
        if !check(*prev_level, *level, increasing) {
            safe = false;
        }

        if (safe == false) && (dampen_triggered == false) {
            dampen_triggered = true;
            safe = true;
        } else {
            prev_level = &level;
        }
    }

    (safe, dampen_triggered)
}

fn check(prev_level: u32, level: u32, increasing:bool) -> bool {
    if prev_level.abs_diff(level) > 3 || prev_level.abs_diff(level) < 1 {
        false
    } else if (prev_level < level) && (!increasing) {
        false
    } else if (prev_level > level) && (increasing) {
        false
    } else {
        true
    }
}

fn is_increasing(prev_level: &u32, level: &u32) -> bool {
    prev_level < level
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    
    #[test]
    fn test_example() {
        assert_eq!(aoc(EXAMPLE.to_string()), (2, 4))
    }

    const HARD_EXAMPLE: &str = "48 46 47 49 51 54 56
1 1 2 3 4 5
1 2 3 4 5 5
5 1 2 3 4 5
1 4 3 2 1
1 6 7 8 9
1 2 3 4 3
9 8 7 6 7
7 10 8 10 11
29 28 27 25 26 25 22 20";
    
    #[test]
    fn test_example_hard() {
        assert_eq!(aoc(HARD_EXAMPLE.to_string()), (0, 10))
    }
}