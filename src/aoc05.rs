use std::collections::{HashMap, HashSet};

pub fn aoc(input: String) -> (i32, i32) {
    let (input_rules, input_updates) = input.split_once("\n\n").unwrap();

    let mut rules:HashMap<i32, HashSet<i32>> = HashMap::new();
    for line in input_rules.lines() {
        let (left, right) = line.split_once("|").unwrap();
        let key = left.parse::<i32>().unwrap();
        let value = right.parse::<i32>().unwrap();
        rules.entry(key).or_insert_with(HashSet::new).insert(value);
    }
    
    let mut updates:Vec<Vec<i32>> = Vec::new();
    for line in input_updates.lines() {
        let update: Vec<i32> = line.split(",")
                        .map(|s| s.parse::<i32>().unwrap())
                        .collect();
        updates.push(update);
    }

    let mut sum: i32 = 0;
    for update in updates {
        let mut pages = update.iter();
        let mut page = pages.next().unwrap();
        let mut correct = 0;
        for next_page in pages {
            if let Some(value) = rules.get(page) {
                if value.contains(next_page) {
                    correct += 1;
                }
                else {
                    break;
                }
            } else {
                break;
            }
            page = next_page;
        }

        if correct == (update.len() - 1) {
            sum += update.get(update.len()/2).unwrap();
        }
    }

    (sum, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    
    #[test]
    fn test_example() {
        assert_eq!(aoc(EXAMPLE.to_string()), (143, 0))
    }
}