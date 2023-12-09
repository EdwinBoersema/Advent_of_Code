use std::str::{Lines, FromStr};


#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    pub fn solve(&self) -> u64 {
        let mut winning_configs = 0;
        for t in 0..self.time {
            let time_left = self.time - t;
            let speed = t;
            if speed * time_left > self.distance {
                winning_configs += 1;
            }
        }
        
        winning_configs
    }
}

pub fn get_races(lines: &mut Lines) -> Vec<Race> {
    let time_line = lines.next()
        .expect("There should be a line here");
    let times = time_line[11..]
        .split(' ')
        .filter_map(|time_str| {
            let result = u64::from_str(time_str);
            if result.is_ok() {
                Some(result.unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    let distance_line = lines.next()
        .expect("There should be a line here");
    let distances = distance_line[11..]
        .split(' ')
        .filter_map(|dst_str| {
            let result = u64::from_str(dst_str);
            if result.is_ok() {
                Some(result.unwrap())
            } else {
                None
            }
        })
        .collect::<Vec<u64>>();

    let mut races: Vec<Race> = Vec::new();
    for (i, time) in times.iter().enumerate() {
        races.push(Race { 
            time: *time, 
            distance: distances[i], 
        });
    }
    races
}

pub fn get_race(lines: &mut Lines) -> Race {
    let time_line = lines.next()
        .expect("First Line should exist");

    let time = time_line[11..]
        .replace(" ", "")
        .parse::<u64>()
        .expect("Time should be numeric");
    println!("time: {}", time);
    
    let distance_line = lines.next()
        .expect("Second Line should exist");

    let distance = distance_line[11..]
        .replace(" ", "")
        .parse::<u64>()
        .expect("Distance should be numeric");
    println!("distance: {}", distance);

    Race { 
        time, 
        distance 
    }
}

pub fn get_score(races: Vec<Race>) -> u64 {
    races
        .iter()
        .map(|race| race.solve())
        .reduce(|acc, count| acc * count)
        .expect("Should return an integer")
}

#[cfg(test)]
mod test {
    use crate::utility::{get_score, get_race};

    use super::Race;
    use rstest::rstest;


    #[rstest]
    #[case(Race { time: 7, distance: 9 }, 4)]
    #[case(Race { time: 15, distance: 40 }, 8)]
    #[case(Race { time: 30, distance: 200 }, 9)]
    fn test_race_solve(
        #[case] input: Race,
        #[case] expected: u64,
    ) {
        assert_eq!(expected, input.solve());
    }

    #[test]
    fn test_get_score() {
        let races = vec![
            Race {
                time: 7,
                distance: 9,
            },
            Race {
                time: 15,
                distance: 40,
            },
            Race {
                time: 30,
                distance: 200,
            },
        ];

        assert_eq!(288, get_score(races));
    }

    #[test]
    fn test_get_race() {
        let mut lines = 
"Time:      7  15   30
Distance:  9  40  200".lines();
        let race = get_race(&mut lines);
        assert_eq!(71530, race.time);
        assert_eq!(940200, race.distance);
    }
}