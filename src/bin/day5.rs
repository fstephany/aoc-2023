use std::{
    cmp::{max, min},
    ops::Range,
};

fn main() {
    let file_content = std::fs::read_to_string("inputs/day5").unwrap();
    println!("Part 1: {}", part_one(&file_content));
    println!("Part 2: {}", part_two(&file_content));
}

#[derive(Default, Debug)]
struct Almanach {
    pub categories: Vec<CategoryMapping>,
}

impl Almanach {
    pub fn traverse(&self, start: u64) -> u64 {
        self.categories
            .iter()
            .fold(start, |acc, category| category.traverse(acc))
    }

    pub fn traverse_with_range(&self, seed: Range<u64>) -> Vec<Range<u64>> {
        // for category in self.categories.iter() {
        //     let mut next = Vec::new();
        //     for range in current {
        //         next.append(&mut category.traverse_with_range(range));
        //     }
        //     current = next;
        // }

        self.categories.iter().fold(vec![seed], |acc, category| {
            let mut all_ranges_mapped = Vec::new();
            for range_to_map in acc {
                all_ranges_mapped.append(&mut category.traverse_with_range(range_to_map));
            }

            all_ranges_mapped
        })
    }
}

#[derive(Default, Debug)]
struct CategoryMapping {
    pub name: String,
    pub mappings: Vec<Mapping>,
}

impl CategoryMapping {
    pub fn traverse(&self, input: u64) -> u64 {
        for mapping in &self.mappings {
            if let Some(mapped) = mapping.map(input) {
                return mapped;
            }
        }
        // No mapping so just return the input
        return input;
    }

    pub fn traverse_with_range(&self, range: Range<u64>) -> Vec<Range<u64>> {
        // Go through all the mappings to to try to cover the range we received.
        let mapped_ranges: Vec<Range<u64>> = self
            .mappings
            .iter()
            .filter_map(|mapping| mapping.map_full_range(&range))
            .collect();

        if mapped_ranges.is_empty() {
            println!(
                "Nothing overlaps {range:?} in {}. Returns it as-is.",
                self.name
            );
        } else {
            println!("Mapping: {range:?} to: {mapped_ranges:?}");
        }

        mapped_ranges
    }
}

#[derive(Debug)]
struct Mapping {
    lower_bound: u64,
    upper_bound: u64,
    destination_lower_bound: u64,
}

impl Mapping {
    pub fn new(dest: u64, source: u64, range: u64) -> Self {
        Self {
            lower_bound: source,
            upper_bound: source + range,
            destination_lower_bound: dest,
        }
    }

    pub fn map(&self, input: u64) -> Option<u64> {
        (input >= self.lower_bound && input < self.upper_bound).then(|| {
            let offset = input - self.lower_bound;
            self.destination_lower_bound + offset
        })
    }

    pub fn map_full_range(&self, input: &Range<u64>) -> Option<Range<u64>> {
        // Return the map of the intersection between the requested range and
        // the range we have.
        // FIXME: Handle the remainder. Because we miss a good chunk of input!

        let potential_intersect = Range {
            start: max(self.lower_bound, input.start),
            end: min(self.upper_bound, input.end),
        };

        if potential_intersect.is_empty() {
            None
        } else {
            // Do the actual mapping
            Some(Range {
                start: potential_intersect.start - self.lower_bound + self.destination_lower_bound,
                end: potential_intersect.end - self.lower_bound + self.destination_lower_bound,
            })
        }
    }
}

fn part_one(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();
    dbg!(&seeds);

    let mut almanach = Almanach::default();
    let mut current_category = CategoryMapping::default();

    for line in lines.map(|l| l.trim()) {
        if line.is_empty() {
            // dbg!(&current_category);
            if !current_category.mappings.is_empty() {
                almanach.categories.push(current_category)
            }

            // We start a new map definition
            current_category = CategoryMapping::default();
        } else if line.starts_with(|c: char| c.is_ascii_alphabetic()) {
            current_category.name = line.to_owned();
        } else if line.starts_with(|c: char| c.is_ascii_digit()) {
            let mut parts = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap());

            current_category.mappings.push(Mapping::new(
                parts.next().unwrap(), // Destination
                parts.next().unwrap(), // Source
                parts.next().unwrap(), // Range
            ))
        } else {
            unreachable!("Invalid input: '{line}'");
        }
    }

    // Handle last line. Ugly but hey.
    almanach.categories.push(current_category);

    // dbg!(&almanach);

    let end_results: Vec<u64> = seeds
        .into_iter()
        .map(|seed| {
            // do the full path
            almanach.traverse(seed)
        })
        .collect();

    dbg!(&end_results);
    end_results.into_iter().min().unwrap()
}

fn part_two(input: &str) -> u64 {
    let mut lines = input.lines();

    let seeds_data: Vec<u64> = lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut almanach = Almanach::default();
    let mut current_category = CategoryMapping::default();

    for line in lines.map(|l| l.trim()) {
        if line.is_empty() {
            // dbg!(&current_category);
            if !current_category.mappings.is_empty() {
                almanach.categories.push(current_category)
            }

            // We start a new map definition
            current_category = CategoryMapping::default();
        } else if line.starts_with(|c: char| c.is_ascii_alphabetic()) {
            current_category.name = line.to_owned();
        } else if line.starts_with(|c: char| c.is_ascii_digit()) {
            let mut parts = line
                .split_ascii_whitespace()
                .map(|n| n.parse::<u64>().unwrap());

            current_category.mappings.push(Mapping::new(
                parts.next().unwrap(), // Destination
                parts.next().unwrap(), // Source
                parts.next().unwrap(), // Range
            ))
        } else {
            unreachable!("Invalid input: '{line}'");
        }
    }

    // Handle last line. Ugly but hey.
    almanach.categories.push(current_category);

    let all_seed_ranges: Vec<Range<u64>> = seeds_data
        .chunks(2)
        .map(|chunk| (chunk[0]..chunk[0] + chunk[1]))
        .collect();

    dbg!(&all_seed_ranges);

    all_seed_ranges
        .into_iter()
        .flat_map(|seed_range| almanach.traverse_with_range(seed_range).into_iter())
        .map(|ranges| {
            dbg!(&ranges);
            ranges
        })
        .min_by_key(|r| r.start)
        .unwrap()
        .start

    // let end_results: Vec<u64> = seeds
    //     .into_iter()
    //     .map(|seed| {
    //         // do the full path
    //         almanach.traverse(seed)
    //     })
    //     .collect();

    // dbg!(&end_results);

    // end_results.into_iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_part_1() {
        let input = "\
        seeds: 79 14 55 13

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
        56 93 4";
        assert_eq!(part_one(input), 35);
    }

    #[test]
    fn real_part1() {
        let input = std::fs::read_to_string("inputs/day5").unwrap();
        assert_eq!(part_one(&input), 226172555);
    }

    #[test]
    fn example_part_2() {
        let input = "\
        seeds: 79 14 55 13

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
        56 93 4";
        assert_eq!(part_two(input), 46);
    }

    #[test]
    fn can_map_one_range() {
        let input = "\
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48";

        // Seeds:
        // - (79..93)
        // - (55..68)

        // Seed-to-soil
        // - (98..100) --> (50..52)
        // - (50..98)  --> (52..100)

        // Smallest is 57. We get there by the seed 55.
        assert_eq!(part_two(input), 57)
    }

    #[test]
    fn can_map_default_when_no_intersects() {
        let input = "\
        seeds: 10 40 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48";

        // Seeds:
        // - (10..49)
        // - (79..99)
        // - (55..68)

        // Seed-to-soil
        // - (98..100) --> (50..52)
        // - (50..98)  --> (52..100)

        // Smallest is 10. We get there by the seed 10.
        assert_eq!(part_two(input), 10)
    }
}
