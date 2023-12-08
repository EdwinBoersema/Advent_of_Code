use std::{collections::HashMap, time::SystemTime, io::{self, Write}};

pub fn process(input: &str) -> u64 {
    /*
        input contains maps to convert from a > b
        each lines contains 3 numbers: destination range start, source range start, range length
        so for seed-to-soil map:
            50 98 2
            52 50 48
        the first line source range starts at 98 and has 2 values, 98 and 99
        destination range starts at 50 and contains 50 and 51
        so converted this gives:
        98 50
        99 51

        2nd line :
        destination range   50, 51, ..., 96, 97
        source range        52, 53, ..., 98, 99
        converted this gives: 
        50 52
        51 53
        52 54 etc

        any sources not mapped retain their value
    */
    // construct the data map and populate it with a vector containing all the seeds to be planted
    println!("Building seed list");
    let seed_list = input.lines()
        .next()
        .expect("1st line should exist")
        .strip_prefix("seeds: ")
        .expect("should start with 'seeds: '")
        .split(' ')
        .map(|s| s.parse::<u64>().expect("should be able to parse to u64"))
        .collect::<Vec<u64>>();
    let mut data_list = Vec::new();
    data_list.push(seed_list);
    println!("Building done");

    // construct the conversion lists
    let mut input_list: Vec<Vec<String>> = Vec::new();
    let mut tmp: Vec<String> = Vec::new();
    println!("Building input list");
    for line in input.lines() {
        if line.is_empty() {
            // add tmp to converstion_list if not empty and clear tmp afterwards
            if !tmp.is_empty() {
                input_list.push(tmp.to_vec());
                tmp.clear();
            }
            // then skip to the next line
            continue;
        } else if (line.chars().last().expect("should exist") == ':') |
                    line.starts_with("seeds:") {
            // skip the first line and decorator lines
            continue;
        } else {
            tmp.push(line.to_string());
        }
    }
    println!("Building done");
    
    // build the converstion maps
    let conversion_maps = input_list.iter()
        .map(|v| build_conversion_map(v.to_vec()))
        .collect::<Vec<HashMap<u64, u64>>>();

    for (index, conversion_map) in conversion_maps.iter().enumerate() {
        // for indexes: 0 = soil, 1 = fertilizer, 2 = water, 3 = light, 4 = temperature, 5 = humidity, 6 = location
        // retrieve the previous list
        let nums_to_check = data_list[index].to_vec();
        // retrieve the converted value from the conversion map if it exist
        // if not, retain current value
        let tmp = nums_to_check.iter()
            .map(|v| *conversion_map.get(v).or(Some(v)).unwrap())
            .collect::<Vec<u64>>();
        
        data_list.push(tmp.to_vec());
    }

    // return lowest location
    *data_list.iter()
        .last()
        .unwrap()
        .iter()
        .min()
        .unwrap()
}

fn build_conversion_map(vector: Vec<String>) -> HashMap<u64, u64> {
    let mut map: HashMap<u64, u64> = HashMap::new();

    for string in vector {
        println!("parsing mapping data");
        let time = SystemTime::now();
        let split = string.split(' ')
            .map(|s| s.parse::<u64>().expect("should be a number"))
            .collect::<Vec<u64>>();
        let dst_start = split[0];
        let src_start = split[1];
        let range = split[2];
        println!("seconds needed for parsing: {}", time.elapsed().unwrap().as_secs());
        print!("Building conversion map: ");
        io::stdout().flush().unwrap();
        for x in 0..range {
            map.insert(
                src_start + x,
                dst_start + x
            );
            if x%1000_000 == 0 {
                print!("x");
                io::stdout().flush().unwrap();
            }
        }
        println!();
    }

    map
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../test.txt");
        assert_eq!(35, process(input));
    }

    #[test]
    fn test_build_conversion_map() {
        let input = vec!["50 98 2".to_string(), "52 50 8".to_string()];

        let expected: HashMap<u64, u64> = HashMap::from([
            (98, 50),
            (99, 51),
            (50, 52),
            (51, 53),
            (52, 54),
            (53, 55),
            (54, 56),
            (55, 57),
            (56, 58),
            (57, 59),
        ]);
        assert_eq!(expected, build_conversion_map(input));
    }
}