use std::cmp;
use std::str::FromStr;

#[derive(Debug)]
struct Ranges {
    source: u64,
    destination: u64,
    length: u64,
}

impl Ranges {
    pub fn new(source: u64, destination: u64, length: u64) -> Self {
        Self { source, destination, length }
    }
}

pub fn execute_solution() {
    let file_str = include_str!("..\\input\\day-five-input.txt");
    let mut split: Vec<&str> = file_str.split("\r\n\r\n").collect();

    let seeds: Vec<u64> = split
        .first()
        .unwrap().clone()
        .split(':')
        .nth(1).unwrap()
        .split_whitespace()
        .map(|v| u64::from_str(v).unwrap())
        .collect();

    let mut seeds_range: Vec<(u64, u64)> = vec![];

    for i in (0..seeds.len()).step_by(2) {
        seeds_range.push(
            (seeds.get(i).unwrap().clone(),
             seeds.get(i).unwrap() + seeds.get(i + 1).unwrap()))
    }

    println!("{:?}", seeds_range);

    split.remove(0);

    println!("Part one solution: {}", part_one(&split, seeds));
    println!("Part two solution: {}", part_two(seeds_range, split));
}

fn part_two(seeds_range: Vec<(u64, u64)>, split: Vec<&str>) -> u64 {
    let mut values_to_iter: Vec<(u64, u64)> = seeds_range.clone();
    // let mut solution = vec![];
    for map in split {
        let ranges: Vec<Ranges> = generate_ranges(map);

        let mut location_values = vec![];
        get_part_two_overlap(values_to_iter.clone(), ranges, &mut location_values);

        values_to_iter = location_values;
    }

    values_to_iter.sort();
    values_to_iter.first().unwrap().0
}

fn get_part_two_overlap(mut seeds: Vec<(u64, u64)>, ranges: Vec<Ranges>, solution: &mut Vec<(u64, u64)>) {
    if seeds.is_empty() { return }
    let (left, right) = seeds.pop().unwrap();

    let mut is_in_range = false;

    for range in &ranges {
        let start = cmp::max(left, range.destination);
        let end = cmp::min(right, range.destination + range.length);
        if start < end {
            solution.push(
                (start - range.destination + range.source,
                 end - range.destination + range.source)
            );
            if start > left { seeds.push((left, start)) }
            if right > end { seeds.push((end, right)) }
            is_in_range = true;
            break;
        }
    }

    if !is_in_range {
        solution.push((left, right));
    }

    return get_part_two_overlap(seeds, ranges, solution)
}

fn part_one(split: &Vec<&str>, seeds: Vec<u64>) -> u64 {
    let mut values_to_iter: Vec<u64> = seeds.clone();
    for map in split {
        let ranges: Vec<Ranges> = generate_ranges(map);

        let mut location_values = vec![];
        for seed in values_to_iter {
            let mut is_in_range = false;

            for range in &ranges {
                if range.destination <= seed && seed < range.destination + range.length {
                    location_values.push(seed - range.destination + range.source);
                    is_in_range = true;
                    break;
                }
            }

            if !is_in_range {
                location_values.push(seed.clone());
            }
        }
        values_to_iter = location_values;
    }

    values_to_iter.sort();
    values_to_iter.first().unwrap().clone()
}

fn generate_ranges(map: &str) -> Vec<Ranges> {
    let mut ranges = vec![];
    for line in map.split("\r\n") {
        let split: Vec<&str> = line.split_whitespace().collect();
        if !split.get(0).unwrap().chars().next().unwrap().is_ascii_digit() { continue; }

        ranges.push(
            Ranges::new(
                u64::from_str(split.get(0).unwrap()).unwrap(),
                u64::from_str(split.get(1).unwrap()).unwrap(),
                u64::from_str(split.get(2).unwrap()).unwrap(),
            )
        )
    }

    ranges
}