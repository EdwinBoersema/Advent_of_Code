pub fn process(input: &str) -> i32 {
    // combine the first digit and last digit to form a single two digit number a.k.a. the calibration value
    let mut calibration_values = vec![];
    for line in input.lines() {
        let mut value = String::new();
        let mut first_pass = true;
        for c in line.chars() {
            if c.is_numeric() {
                if first_pass {
                    value.push(c);
                    value.push(c);
                    first_pass = false;
                    continue;
                }
                value.pop();
                value.push(c);
            }
        }
        calibration_values.push(value);
    }

    // sum all calibration values
    let mut result = 0;
    for i in calibration_values.iter() {
        result += i.parse::<i32>().unwrap();
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        assert_eq!(142, process(input));
    }
}