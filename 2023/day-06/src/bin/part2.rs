use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u64 {
    let re = Regex::new(r"\s+").unwrap();
    let lines: Vec<String> = input.lines()
        .map(|l| l.split(':').last().unwrap())
        .map(|s| re.replace_all(s, "").trim().to_string())
        .collect();
    let time: u64 = lines[0].parse().unwrap();
    let distance: u64 = lines[1].parse().unwrap();
    println!("{:?}", time);
    println!("{:?}", distance);
    
    let mut counter = 0;
    (0..time).for_each(|speed| {
        if speed * (time-speed) > distance {
            counter += 1;
        }
    });

    counter
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part2("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 71503);
    }
}
