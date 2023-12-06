use std::collections::BTreeMap;

use itertools::Itertools;

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

pub fn process(input: &str) -> u32 {
    // search through input for numbers
    // collect numbers
    // search adjacent spaces for symbols
    // if symbol is adjacent, add number to list
    // sum up list
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(
                move |(x, character)| {
                    (
                (y as i32,x as i32),
                match character {
                    '.' => Value::Empty,
                    c if c.is_ascii_digit() => {
                        Value::Number(
                            c.to_digit(10).expect(
                                "should be a number",
                            ),
                        )
                    }
                    c => Value::Symbol(c),
                },
            )
                },
            )
        })
        .collect::<BTreeMap<(i32, i32), Value>>();

    // construct the list with numbers
    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((y, x), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last();
                    match last_num {
                        Some(((last_num_x, _), _)) => {
                            if last_num_x + 1 == *x {
                                let last = numbers
                                    .iter_mut()
                                    .last()
                                    .expect("should exist");
                                last.push(((*x, *y), *num))
                            } else {
                                numbers.push(vec![(
                                    (*x, *y),
                                    *num,
                                )]);
                            }
                        },
                        None => unimplemented!("Should not happen")
                    }
                },
                //
                None => {
                    numbers.push(vec![((*y, *x), *num)]);
                }
            }
        }
    }

    // check for symbols adjacent to the numbers
    let mut counter = 0;
    for number_list in numbers {
        // x & y positions
        let positions = [
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
        ];
        let num_positions: Vec<(i32, i32)> = number_list
            .iter()
            .map(|((y, x), _)| (*x, *y))
            .collect();
        let position_to_check: Vec<(i32, i32)> = number_list
            .iter()
            .flat_map(|(pos, _)| {
                positions.iter().map(|outer_pos| {
                    // outer_pos.x + pos.x, .y + .y
                    (
                        outer_pos.0 + pos.1,
                        outer_pos.1 + pos.0,
                    )
                })
            })
            .unique()
            .filter(|num| !num_positions.contains(num))
            .collect();

        // check if the numbers in number_list are part numbers
        let is_part_number =
            position_to_check.iter().any(|pos| {
                let value = map.get(pos);
                #[allow(clippy::match_like_matches_macro)]
                if let Some(Value::Symbol(_)) = value {
                    true
                } else {
                    false
                }
            });

        if is_part_number {
            counter += number_list
                .iter()
                .map(|(_, num)| num.to_string())
                .collect::<String>()
                .parse::<u32>()
                .unwrap()
        }
    }
        
    counter
}

// fn search_for_numbers(line: &str) -> Vec<u32> {
//     let mut result: Vec<u32> = Vec::new();
//     // go through line, collect numbers
//     let mut temp = String::new();
//     for (_i, c) in line.char_indices() {
//         if c.is_numeric() {
//             temp.push(c)
//         } else {
//             result.push(
//                 temp.parse::<u32>().unwrap()
//             );
//         }
//     }

//     result
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..";
        assert_eq!(4361, process(input));
    }
}