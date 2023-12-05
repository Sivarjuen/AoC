fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug)]
struct Gear {
    pos: (u32, u32),
    adj: u32,
    val: u32,
}

fn part2(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut symbol_map: Vec<Vec<u32>> = vec![];
    for line in lines.iter() {
        symbol_map.push(line.chars().map(is_gear).collect());
    }

    let mut gears: Vec<Gear> = vec![];
    for (iindex, symbol_line) in symbol_map.iter().enumerate() {
        for (jindex, symbol) in symbol_line.iter().enumerate() {
            if *symbol == 1 {
                let gear = Gear {
                    pos: (iindex as u32, jindex as u32),
                    adj: 0,
                    val: 1,
                };
                gears.push(gear);
            }
        }
    }

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
        for np in number_pos {
            let adj_gears =
                check_surrounding(row, np.0, np.1.to_string().len() as u32, &symbol_map);
            for ag in adj_gears {
                for mut g in &mut gears {
                    if g.pos == ag {
                        g.adj += 1;
                        g.val *= np.1;
                    }
                }
            }
        }
    }
    gears.iter().filter(|g| g.adj == 2).map(|g| g.val).sum()
}

fn is_gear(c: char) -> u32 {
    if c == '*' {
        1
    } else {
        0
    }
}

fn check_surrounding(
    row: usize,
    index: u32,
    length: u32,
    symbol_map: &Vec<Vec<u32>>,
) -> Vec<(u32, u32)> {
    let mut adj_gears: Vec<(u32, u32)> = vec![];
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
            if symbol_map[i][j as usize] == 1 {
                adj_gears.push((i as u32, j));
            }
        }
    }
    adj_gears
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part2(
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
        assert_eq!(result, 467835);
    }
}
