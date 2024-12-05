pub fn aoc(input: String) -> (i32, i32) {
    let search: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word = vec!['X', 'M', 'A', 'S'];
    let x_search: Vec<i32> = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let y_search: Vec<i32> = vec![-1, 0, 1, -1, 1, -1, 0, 1];
    let mut count = 0;
    let mut count_p2 = 0;

    let x_len = search.len();
    let y_len = search[0].len();
    for x in 0..x_len {
        for y in 0..y_len {
            if word[0] == search[x][y] {
                // Search in all eight directions
                for d in 0..8 {
                    for w in 1..word.len() {
                        let m;
                        let n;
                        if x_search[d] < 0 {
                            m = x.checked_sub(1 * w).unwrap_or(x_len);
                        } else if x_search[d] == 0{
                            m = x;
                        } else {
                            m = x.checked_add(1 * w).unwrap_or(x_len);
                        }
                        if m > x_len-1 {
                            break;
                        }

                        if y_search[d] < 0 {
                            n = y.checked_sub(1 * w).unwrap_or(y_len);
                        } else if y_search[d] == 0{
                            n = y;
                        } else {
                            n = y.checked_add(1 * w).unwrap_or(y_len);
                        }
                        if n > y_len-1 {
                            break;
                        }

                        if word[w] != search[m][n] {
                            break;
                        }

                        if w == word.len()-1 {
                            count += 1;
                        }
                    }
                }
            }
            
            // Search for the X [MAS]
            if (x != 0) && (x != x_len-1) && (y != 0) && (y != y_len-1) {
                if 'A' == search[x][y] {
                    let mut num_m = 0;
                    let mut num_s = 0;
                    if search [x-1][y-1] == 'M' {
                        if search [x+1][y+1] != 'M' {
                            num_m += 1;
                        }
                    }
                    if search [x+1][y-1] == 'M' {
                        if search [x-1][y+1] != 'M' {
                            num_m += 1;
                        }
                    }
                    if search [x-1][y+1] == 'M' {
                        if search [x+1][y-1] != 'M' {
                            num_m += 1;
                        }
                    }
                    if search [x+1][y+1] == 'M' {
                        if search [x-1][y-1] != 'M' {
                            num_m += 1;
                        }
                    }
                    if search [x-1][y-1] == 'S' {
                        if search [x+1][y+1] != 'S' {
                            num_s += 1;
                        }
                    }
                    if search [x+1][y-1] == 'S' {
                        if search [x-1][y+1] != 'S' {
                            num_s += 1;
                        }
                    }
                    if search [x-1][y+1] == 'S' {
                        if search [x+1][y-1] != 'S' {
                            num_s += 1;
                        }
                    }
                    if search [x+1][y+1] == 'S' {
                        if search [x-1][y-1] != 'S' {
                            num_s += 1;
                        }
                    }
                    if num_m == 2 && num_s == 2 {
                        count_p2 += 1;
                    }
                }
            }
        }
    }

    (count, count_p2)
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    
    #[test]
    fn test_example() {
        assert_eq!(aoc(EXAMPLE.to_string()), (18, 9))
    }
}