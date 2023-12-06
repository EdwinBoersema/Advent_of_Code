pub fn process(input: &str) -> u32 {
    let mut total_cards = vec![1; input.lines().count()];

    let mut index = 0;
    for game in input.lines() {
        let matching_numbers = process_game(game);
        // get multiplier for this scratchcard from total_cards
        let multiplier = total_cards[index];
        for x in 0..matching_numbers {
            total_cards[index + x as usize + 1] += multiplier;
        }

        // increase counter
        index += 1;
    }


    total_cards.iter()
        .sum()
}

fn process_game(game: &str) -> u32 {
    // trim whitespace from input string
    let game = game.trim();
    // build prefix to be removed
    let mut prefix = String::new();
    for char in game.chars() {
        if char == ':' {
            prefix.push(char);
            // break after this symbol
            break;
        } else {
            prefix.push(char);
        }
    }
    
    // remove prefix
    let reduced_line = game.trim().strip_prefix(&prefix).unwrap();
    // split game into 2 vectors with numbers
    let strings = reduced_line.split('|')
        .map(|s| s.to_owned())
        .collect::<Vec<String>>();

    let lists = strings.iter()
        .map(|vec| {
            vec
                .split(' ')
                .map(|s| s.trim())
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // return count of matching numbers
    lists[0].iter()
        .map(|num| lists[1].contains(num))
        .filter(|b| *b)
        .count() as u32
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::rstest;

    #[test]
    fn test_process() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(30, process(input));
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 4)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn test_process_game(
        #[case] game: &str,
        #[case] expected: u32,
    ) {
        assert_eq!(expected, process_game(game));
    }
}