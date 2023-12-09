use crate::utility::get_race;

pub fn process(input: &str) -> u64 {
    let mut lines = input.lines();
    let race = get_race(&mut lines);

    race.solve()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../test.txt");
        assert_eq!(71503, process(input));
    }
}