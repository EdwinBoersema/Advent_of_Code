pub fn process(input: &str) -> u32 {
    let result = input
        .lines()
        .map(process_line)
        .sum::<u32>();
    result
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let reduced_line = &line[index..];
        let result = if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)            
        } else {
            reduced_line
                .chars()
                .next()
                .unwrap()
                .to_digit(10)
        };

        result
    });

    let first_digit = it.next().expect("should be a number");

    let last_digit = match it.last() {
        Some(x) => x,
        None => first_digit
    };

    first_digit * 10 + last_digit
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_process() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        assert_eq!(281, process(input));
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    fn test_lines(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_line(line));
    }
}