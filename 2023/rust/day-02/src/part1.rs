pub fn process(input: &str) -> u32 {
    input
        .lines()
        .map(process_line)
        .sum::<u32>()
}

fn process_line(line: &str) -> u32 {
    // processes the line and returns the game index if the game is possible
    let mut id = 0;
    let is_valid = line
        .split(&[':', ';', ','])
        .all(|value| {
        if value.contains("red") {
            value.trim()
                .get(0..2).unwrap()
                .trim()
                .parse::<u32>().unwrap() <= 12
        } else if value.contains("green") {
            value.trim()
                .get(0..2).unwrap()
                .trim()
                .parse::<u32>().unwrap() <= 13
        } else if value.contains("blue") {
            value.trim()
                .get(0..2).unwrap()
                .trim()
                .parse::<u32>().unwrap() <= 14
        } else {
            let value = value.trim();
            id = value[5..].parse::<u32>().unwrap();
            true
        }
    });

    if is_valid {
        id
    } else {
        0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_process() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input));
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 1)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 2)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 0)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 0)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 5)]
    fn test_lines(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_line(line));
    }
}