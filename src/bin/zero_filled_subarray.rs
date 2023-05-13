use syc_leetcode_solution_rs::parse_util;
use syc_leetcode_solution_rs::Solution;

fn main() {
    let buffer = parse_util::read_line().unwrap();
    let (_input, list) = parse_util::parse_list(&buffer).unwrap();
    let nums: Vec<i32> = list.iter().map(|s| s.parse().unwrap()).collect();
    println!("{}", Solution::zero_filled_subarray(nums));
}
