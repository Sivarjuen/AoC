use std::cmp;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    let lines = input.lines().map(|l| l.split_whitespace());
    let mut valid_games: Vec<u32> = vec![];
    for game in lines {
        let list: Vec<&str> = game.skip(1).collect();
        let game_id: u32 = list[0]
            .chars()
            .take(list[0].len() - 1)
            .collect::<String>()
            .parse()
            .unwrap();
        let mut index = 1;

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        while index < list.len() {
            let value: u32 = list[index].parse().unwrap();
            let colour = list[index + 1];
            if colour.contains("red") {
                max_red = cmp::max(max_red, value);
            } else if colour.contains("green") {
                max_green = cmp::max(max_green, value);
            } else if colour.contains("blue") {
                max_blue = cmp::max(max_blue, value);
            }

            index += 2;
        }
        if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
            valid_games.push(game_id)
        }
    }
    valid_games.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = part1(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        );
        assert_eq!(result, 8);
    }
}
