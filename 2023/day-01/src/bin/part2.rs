use phf::phf_map;

static DIGITS: phf::Map<&str, char> = phf_map! {
    "one" => '1',
    "two" => '2',
    "three" => '3',
    "four" => '4',
    "five" => '5',
    "six" => '6',
    "seven" => '7',
    "eight" => '8',
    "nine" => '9',
};

fn main() {
    let input = include_str!("./input.txt");
    let output = task(input);
    dbg!(output);
}

fn task(input: &str) -> u32 {
    let mut numbers: Vec<u32> = vec![];
    let parts = input.split('\n');
    for part in parts {
        let mut digit_pos: Vec<(usize, char)> = vec![];
        for digit in DIGITS.keys() {
            let v: Vec<(usize, &str)> = part.match_indices(digit).collect();
            for pos in v.iter() {
                digit_pos.push((pos.0, *DIGITS.get(pos.1).unwrap()));
            }
        }
        for (i, c) in part.chars().enumerate() {
            if c.is_ascii_digit() {
                digit_pos.push((i, c));
            }
        }
        let mut min_idx = 0;
        let mut min_value = '-';
        let mut max_idx = 0;
        let mut max_value = '-';

        for p in digit_pos {
            if min_value == '-' {
                min_idx = p.0;
                min_value = p.1;
                max_idx = p.0;
                max_value = p.1;
            } else if p.0 < min_idx {
                min_idx = p.0;
                min_value = p.1;
            } else if p.0 > max_idx {
                max_idx = p.0;
                max_value = p.1;
            }
        }

        if min_value != '-' {
            let part_number: u32 = format!("{}{}", min_value, max_value).parse().unwrap();
            numbers.push(part_number);
        }
    }

    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = task(
            "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen",
        );
        assert_eq!(result, 281);
    }
}
