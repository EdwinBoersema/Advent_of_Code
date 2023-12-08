use crate::utility::{Id, get_seed_numbers, get_mappings, get_min_location_id};

pub fn process(input: &str) -> Id {
    let mut input_lines = input.lines();
    let seeds = get_seed_numbers(&mut input_lines);
    let mappings = get_mappings(&mut input_lines);

    // convert seeds to ranges of seeds
    let mut seed_ranges = Vec::new();
    let mut start: Id = 0;
    // get 1st number, add Id's for the range length
    for (i, &seed) in seeds.iter().enumerate() {
        if i%2 == 0 {
            start = seed;
        } else {
            // push seed range into the vector
            seed_ranges.push(start..(start + seed));
        }
    }

    seed_ranges
        .into_iter()
        .map(|r| get_min_location_id(r.into_iter(), &mappings))
        .min()
        .expect("Cannot find a minimum id from the seed ranges")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../test.txt");
        assert_eq!(46, process(input));
    }
}