pub fn process(input: &str) -> u32 {
    // go through each game (line)
    // gather the minimum occurence of each colour
    // multiply the minima in each game
    // sum up the powers of all the games
    // return sum
    input
        .lines()
        .map(process_line)
        .sum::<u32>()
}

fn process_line(line: &str) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    // loop through sub slices
    line.split(&[':', ';', ','])
        .for_each(|value| {
            if value.contains("red") {
                let val = value.trim()
                    .get(0..2).unwrap()
                    .trim()
                    .parse::<u32>().unwrap();
                if val > min_red { min_red = val}
            } else if value.contains("green") {
                let val = value.trim()
                    .get(0..2).unwrap()
                    .trim()
                    .parse::<u32>().unwrap();
                if val > min_green { min_green = val}
            } else if value.contains("blue") {
                let val = value.trim()
                    .get(0..2).unwrap()
                    .trim()
                    .parse::<u32>().unwrap();
                if val > min_blue { min_blue = val}
            }
        });

    // multiple minimum values
    min_red * min_green * min_blue
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
        assert_eq!(2286, process(input));
    }

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 48)]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12)]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 1560)]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 630)]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 36)]
    fn test_line(
        #[case] line: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_line(line));
    }
}