fn main() {
    let file_content = std::fs::read_to_string("inputs/day5").unwrap();
    println!("Part 1: {}", part_one(&file_content));
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
        (input >= self.lower_bound && input <= self.upper_bound).then(|| {
            let offset = input - self.lower_bound;
            self.destination_lower_bound + offset
        })
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
            dbg!(&current_category);
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
}
