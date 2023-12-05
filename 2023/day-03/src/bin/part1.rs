fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    let lines: Vec<&str> = input.lines().collect();
    let mut symbol_map: Vec<Vec<bool>> = vec![];
    for line in lines.iter() {
        symbol_map.push(line.chars().map(is_symbol).collect());
    }

    println!("{:?}", symbol_map);
    for (row, line) in lines.iter().enumerate() {
        let c_iter = line.chars();
        let mut number_pos: Vec<(u32, u32)> = vec![];
        let mut current: Vec<char> = vec![];
        let mut start_index = 0;
        for (index, c) in c_iter.enumerate() {
            if !c.is_ascii_digit() {
                if !current.is_empty() {
                    let str: String = current.iter().collect();
                    let number: u32 = str.parse().unwrap();
                    number_pos.push((start_index, number))
                }
                current.clear();
            } else {
                if current.is_empty() {
                    start_index = index as u32;
                }
                current.push(c);
            }
        }
        if !current.is_empty() {
            let str: String = current.iter().collect();
            let number: u32 = str.parse().unwrap();
            number_pos.push((start_index, number))
        }
        println!("{:?}", number_pos);
        for np in number_pos {
            if check_surrounding(row, np.0, np.1.to_string().len() as u32, &symbol_map) {
                sum += np.1
            }
        }
    }
    sum
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

fn check_surrounding(row: usize, index: u32, length: u32, symbol_map: &Vec<Vec<bool>>) -> bool {
    let row_range = if row == 0 {
        row..=row + 1
    } else {
        row - 1..=row + 1
    };
    for i in row_range {
        if i >= symbol_map.len() {
            continue;
        }
        let col_range = if index == 0 {
            index..=index + length
        } else {
            index - 1..=index + length
        };
        for j in col_range {
            if j >= symbol_map[0].len() as u32 {
                continue;
            }
            // println!("Checking {:?}, {:?}", i, j);
            if symbol_map[i][j as usize] {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part1(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        assert_eq!(result, 4361);
    }
}
