pub fn process(input: &str) -> u32 {
    /*
        create 2 arrays, left & right
        sort lists?
        smallest in left, with smallest in right -> tuple array
        continue until done

        sum differences
    */
    let mut left = Vec::new();
    let mut right = Vec::new();
    for line in input.lines() {
        for (i, string) in line.split(' ').enumerate() {
            if string.trim().is_empty() {
                continue;
            }
            let num = string.parse::<u32>().unwrap();
            if i == 0 {
                left.push(num);
            } else {
                right.push(num);
            }
        }
    }

    // sort arrays
    left.sort_unstable();
    right.sort_unstable();

    // create pairs
    let mut pairs: Vec<(u32, u32)> = Vec::new();
    for (i, num) in left.iter().enumerate() {
        let pair = (*num, *right.get(i).unwrap());
        pairs.push(pair);
    }

    let mut sum = 0;
    for (lnum, rnum) in pairs.iter() {
        if lnum > rnum {
            sum = sum + (lnum - rnum);
        } else if lnum < rnum {
            sum = sum + (rnum - lnum);
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!(11, process(input));
    }
}