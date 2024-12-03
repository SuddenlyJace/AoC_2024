use std::collections::HashMap;

pub fn aoc(input: String) -> (i32, i32) {
    // We need to build some vectors 
    let mut location_id_a: Vec<i32> = Vec::new();
    let mut location_id_b: Vec<i32> = Vec::new();
    
    let lines = input.lines();
    for line in lines {
        let mut split = line.split("   ");
        let a = split.next().unwrap();
        let b = split.next().unwrap();
        location_id_a.push(a.parse::<i32>().unwrap());
        location_id_b.push(b.parse::<i32>().unwrap());
    }

    location_id_a.sort();
    location_id_b.sort();

    let location = subtract_elements(&location_id_a, &location_id_b);
    let sum: i32 = location.iter().sum();

    let mut location_hash:HashMap<i32, i32> = HashMap::new();
    for int in location_id_b {
        let value = location_hash.entry(int).or_insert(0);
        *value += 1;
    }

    let mut similarity = 0;
    for int in location_id_a {
        similarity += int * *location_hash.entry(int).or_default();
    }

    (sum, similarity)
}

fn subtract_elements(vec_a: &Vec<i32>, vec_b: &Vec<i32>) -> Vec<i32> {
    vec_a.iter().zip(vec_b).map(|(a,b)| (a-b).abs()).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_example() {
        assert_eq!(aoc(EXAMPLE.to_string()), (11, 31))
    }
}