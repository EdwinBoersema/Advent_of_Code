pub fn process(_input: &str) -> u32 {
    /* 
        Camel Cards
    > list of hands
    > order them based on the strength
    > 
    */
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../test.txt");
        assert_eq!(6440, process(input));
    }
}