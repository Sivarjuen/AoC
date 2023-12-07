fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Bounds {
    min: u64,
    max: u64,
    value: u64
}
impl Bounds {
    fn new(value: u64, min: u64, range: u64) -> Bounds {
        Bounds { min, max: min+range, value }
    }
    fn contains(&self, value: u64) -> bool {
        self.min <= value && value < self.max
    }
    fn get_value(&self, value: u64) -> u64 {
        if self.contains(value) {
            self.value + value - self.min
        } else {
            panic!("Value '{}' is outside of bounds {} to {}", value, self.min, self.max);
        }
    
    }
}

fn parse_number_list(line: &str) -> Vec<u64> {
    line.split(' ').map(|c| c.parse::<u64>().expect("number")).collect()
}

fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let seeds: Vec<u64> = parse_number_list(lines
        .next()
        .unwrap()
        .split("seeds: ")
        .last()
        .expect("seeds"));
    println!("{:?}", seeds);

    let seed_to_soil: Vec<Bounds> = lines
        .by_ref()
        .skip_while(|l| l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_number_list)
        .map(|l| Bounds::new(l[0], l[1], l[2]))
        .collect();

    println!("{:?}", seed_to_soil);

    let soil_to_fertilizer: Vec<Bounds> = lines
    .by_ref()
        .skip_while(|l| l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_number_list)
        .map(|l| Bounds::new(l[0], l[1], l[2]))
        .collect();

    println!("{:?}", soil_to_fertilizer);

    let fertilizer_to_water: Vec<Bounds> = lines
    .by_ref()
        .skip_while(|l| l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_number_list)
        .map(|l| Bounds::new(l[0], l[1], l[2]))
        .collect();

    println!("{:?}", fertilizer_to_water);

    let water_to_light: Vec<Bounds> = lines
    .by_ref()
        .skip_while(|l| l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_number_list)
        .map(|l| Bounds::new(l[0], l[1], l[2]))
        .collect();

    println!("{:?}", water_to_light);

    let light_to_temperature: Vec<Bounds> = lines
    .by_ref()
        .skip_while(|l| l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_number_list)
        .map(|l| Bounds::new(l[0], l[1], l[2]))
        .collect();

    println!("{:?}", light_to_temperature);

    let temperature_to_humidity: Vec<Bounds> = lines
    .by_ref()
        .skip_while(|l| l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_number_list)
        .map(|l| Bounds::new(l[0], l[1], l[2]))
        .collect();

    println!("{:?}", temperature_to_humidity);

    let humidity_to_location: Vec<Bounds> = lines
    .by_ref()
        .skip_while(|l| l.is_empty())
        .skip(1)
        .take_while(|l| !l.is_empty())
        .map(parse_number_list)
        .map(|l| Bounds::new(l[0], l[1], l[2]))
        .collect();

    println!("{:?}", humidity_to_location);

    seeds.into_iter().map(|seed| {
        let soil = seed_to_soil.iter().find(|s| s.contains(seed)).map_or_else(|| seed, |b| b.get_value(seed));
        let fertilizer = soil_to_fertilizer.iter().find(|s| s.contains(soil)).map_or_else(|| soil, |b| b.get_value(soil));
        let water = fertilizer_to_water.iter().find(|s| s.contains(fertilizer)).map_or_else(|| fertilizer, |b| b.get_value(fertilizer));
        let light = water_to_light.iter().find(|s| s.contains(water)).map_or_else(|| water, |b| b.get_value(water));
        let temperature = light_to_temperature.iter().find(|s| s.contains(light)).map_or_else(|| light, |b| b.get_value(light));
        let humidity = temperature_to_humidity.iter().find(|s| s.contains(temperature)).map_or_else(|| temperature, |b| b.get_value(temperature));
        let location = humidity_to_location.iter().find(|s| s.contains(humidity)).map_or_else(|| humidity, |b| b.get_value(humidity));
        // println!("seed {}, soil {}, fertilizer {}, water {}, light {}, temp {}, humidity {}, location {}", seed, soil, fertilizer, water, light, temperature, humidity, location);
        location
    }).min().expect("min")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample() {
        let result = part1("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");
        assert_eq!(result, 35);
    }
}
