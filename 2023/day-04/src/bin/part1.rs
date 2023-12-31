fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let mut result: u32 = 0;
    let lines = input.lines();
    for line in lines {
        let number_part = line
            .split(':')
            .last()
            .expect("number part is present")
            .trim();
        let numbers: Vec<_> = number_part
            .split('|')
            .map(|part| {
                part.split(' ')
                    .filter(|p| !p.is_empty())
                    .map(|p| p.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .collect();
        println!("{:?}", numbers);

        let matches: u32 = numbers[1]
            .iter()
            .filter(|n| numbers[0].contains(n))
            .map(|_x| 1)
            .sum();
        if matches > 0 {
            result += u32::pow(2, matches - 1);
        }
        println!("{:?} matches\n", matches);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part1(
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        );
        assert_eq!(result, 13);
    }
}
