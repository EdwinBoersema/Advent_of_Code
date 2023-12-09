use crate::utility::*;

pub fn process(input: &str) -> u64 {
    /*
    input:
        Time:      7  15   30
        Distance:  9  40  200
    
    hold button down > release
    time = pressing button + boat traveling

    starting speed = 0 millimeters / millisecond
    1 millisecond pressing button = +1 millimeter / millisecond
    check what configurations beat the current record > count

    multiply all counts > return val
    */
    let mut input_iter = input.lines();
    let races: Vec<Race> = get_races(&mut input_iter);

    get_score(races)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../test.txt");
        assert_eq!(288, process(input));
    }
}