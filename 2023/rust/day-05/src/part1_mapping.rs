use crate::utility::*;

pub fn process(input: &str) -> Id {
    let mut input_lines = input.lines();
    let seeds = get_seed_numbers(&mut input_lines);
    let mappings = get_mappings(&mut input_lines);
    get_min_location_id(seeds.iter().copied(), &mappings)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../test.txt");
        assert_eq!(35, process(input));
    }
}