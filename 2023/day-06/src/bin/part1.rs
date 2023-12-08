use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u64 {
    let re = Regex::new(r"\s+").unwrap();
    let lines: Vec<String> = input.lines()
        .map(|l| l.split(':').last().unwrap())
        .map(|s| re.replace_all(s, " ").trim().to_string())
        .collect();
    let times: Vec<u64> = lines[0]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let distances: Vec<u64> = lines[1]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", times);
    println!("{:?}", distances);

    let mut res = 1;
    
    for i in 0..times.len() {
        let mut counter = 0;
        let time = times[i];
        let distance = distances[i];

        (0..time).for_each(|speed| {
            if speed * (time-speed) > distance {
                counter += 1;
            }
        });
        res *= counter;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part1("Time:      7  15   30
Distance:  9  40  200");
        assert_eq!(result, 288);
    }
}
