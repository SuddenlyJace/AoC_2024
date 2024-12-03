pub fn aoc(input: String) -> (i32, i32) {
    // WE NEED A VECTOR OF VECTORS TO CONTAIN ALL THE REPORTS!!!
    let mut reports: Vec<Vec<u32>> = Vec::new();
    
    // Populate our vectors with our data
    // Take a line, convert to a String, then parse to u32, then REMOVE RUST (unwrap). Then make vector out of our iterator.
    // PUSH that report into our reports
    let lines = input.lines();
    for line in lines {
        let report: Vec<u32> = line.split(" ").map(|x| x.to_string().parse::<u32>().unwrap()).collect();
        reports.push(report);
    }

    // Time to measure some levels
    let mut safe_reports: i32 = 0;
    for report in reports {
        let mut optional_previous_level: Option<u32> = None;
        let mut optional_increasing: Option<bool> = None;
        let mut safe: bool = true;
        // Loop over each level in report and compare against previous
        for level in report {
            // Check for first iteration
            if let Some(prev_level) = optional_previous_level {
                // Check for first level comparison of increasing or decreasing
                if let Some(increasing) = optional_increasing {
                    if (prev_level < level) && (!increasing) {
                        safe = false
                    } else if (prev_level > level) && (increasing) {
                        safe = false
                    } else if prev_level == level {
                        safe = false
                    } else {
                        if prev_level.abs_diff(level) > 3 {
                            safe = false
                        }
                    }
                } else {
                    // New increasing or decreasing state
                    if prev_level.abs_diff(level) > 3 {
                        safe = false
                    } else if prev_level < level {
                        optional_increasing = Some(true);
                    } else if prev_level > level {
                        optional_increasing = Some(false);
                    } else {
                        safe = false
                    }
                }
            }
            optional_previous_level = Some(level);
        }

        // We should probaqbly reverse all this logic to shrink it... BUT HERE WE ARE
        if safe {
            safe_reports += 1;
        }
    }

    (safe_reports,0)
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
        assert_eq!(aoc(EXAMPLE.to_string()), (2, 0))
    }
}