fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut numbers: Vec<i32> = vec![];
    let parts = input.split('\n');
    for part in parts {
        let mut first_digit = '-';
        let mut second_digit = '-';
        for c in part.chars() {
            if c.is_ascii_digit() {
                if first_digit == '-' {
                    first_digit = c
                }
                second_digit = c
            }
        }
        if first_digit != '-' {
            let part_number: i32 = format!("{}{}", first_digit, second_digit).parse().unwrap();
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
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, 142);
    }
}
