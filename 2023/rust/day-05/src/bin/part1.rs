// use day_05::part1::process;
use day_05::part1_mapping::process;

fn main() {
    let file = include_str!("../../input1.txt");
    let result = process(file);
    println!("{}", result);
}