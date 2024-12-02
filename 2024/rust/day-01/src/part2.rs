pub fn process(input: &str) -> u32 {
    /*
        split input into 2 arrays
        iter through left array
        multiply by how many times each num exists in right array
        return the sum of similarity scores
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

    let mut similarity = 0;
    let mut sum = 0;
    for lnum in left.iter() {
        // check for same numbers in right array\
        for i in 0..right.len() {
            if lnum == right.get(i).unwrap() {
                similarity += 1;
            }
        }
        // multiply similarity and add to sum
        sum += lnum * similarity;
        similarity = 0;
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
        assert_eq!(31, process(input));
    }
}