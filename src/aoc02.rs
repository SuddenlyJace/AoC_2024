pub fn aoc(input: String) -> (i32, i32) {
    // WE NEED A VECTOR OF VECTORS TO CONTAIN ALL THE REPORTS!!!
    let mut reports: Vec<Vec<u32>> = Vec::new();
    
    // Populate our vectors with our data
    // Take a line, convert to a String, then parse to u32, then REMOVE RUST (unwrap). Then make vector out of our iterator.
    // PUSH that report into our reports
    let lines = input.lines();
    for line in lines {
        let report: Vec<u32> = line
            .split(" ")
            .map(|x| x.to_string().parse::<u32>().unwrap())
            .collect();
        reports
            .push(report);
    }

    // Time to measure some levels
    let mut safe_reports: i32 = 0;
    let mut dampened_reports: i32 = 0;

    for report in reports {
        let mut optional_previous_level: Option<u32> = None;
        let mut _optional_dampen_level: Option<u32> = None;
        let mut optional_increasing: Option<bool> = None;
        let mut safe: bool = true;
        let mut dampen_triggered: bool = false;
        let report_length = report.len();

        // Loop over each level in report and compare against previous
        for (i, level) in report.iter().enumerate() {
            // Check for first iteration
            if let Some(prev_level) = optional_previous_level {
                // Check for first level comparison of increasing or decreasing
                if let Some(increasing) = optional_increasing {
                    if (prev_level < *level) && (!increasing) {
                        safe = false
                    } else if (prev_level > *level) && (increasing) {
                        safe = false
                    } else if prev_level == *level {
                        safe = false
                    } else {
                        if prev_level.abs_diff(*level) > 3 {
                            safe = false
                        }
                    }
                } else {
                    // New increasing or decreasing state
                    if prev_level.abs_diff(*level) > 3 {
                        safe = false
                    } else if prev_level < *level {
                        optional_increasing = Some(true);
                    } else if prev_level > *level {
                        optional_increasing = Some(false);
                    } else {
                        safe = false
                    }
                }
            }

            if (safe == false) && (dampen_triggered == false) {
                dampen_triggered = true;
                safe = true;
                optional_increasing = None;
                if report_length == (i+1) {
                    break;
                } else if i == 1 {
                    optional_previous_level = Some(*level);
                }
            } else {
                optional_previous_level = Some(*level);
                _optional_dampen_level = optional_previous_level;
            }
        }

        // We should probably reverse all this logic to shrink it... BUT HERE WE ARE
        if safe && (dampen_triggered == false) {
            safe_reports += 1;
            dampened_reports += 1;
            //println!("{:?}", report);
        } else if safe {
            dampened_reports += 1;
           // println!("{:?}", report);
        }
    }

    (safe_reports, dampened_reports)
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

    fn test_example_hard() {
        assert_eq!(aoc(HARD_EXAMPLE.to_string()), (0, 10))
    }
}