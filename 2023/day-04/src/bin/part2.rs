fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    "todo".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part2("");
        assert_eq!(result, "todo".to_string());
    }
}
