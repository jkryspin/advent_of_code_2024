use std::collections::{HashMap, VecDeque};
use cached::proc_macro::cached;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let solver = Solver::from(input.to_string());
    Some(solver.solve(true))


}

pub fn part_two(input: &str) -> Option<u64> {
    let solver = Solver::from(input.to_string());
    Some(solver.solve(false))
}

#[derive(Debug)]
struct Solver{
    towel_patterns: Vec<String>,
    towels: Vec<String>,
}

impl Solver{
    fn solve(&self, part_one:bool) -> u64{
        let mut count = 0;
        for towel in &self.towels{
            if part_one{
                if ways(self.towel_patterns.clone(), towel.clone()) > 0{
                    count += 1;
                }
            }else {
                count += ways(self.towel_patterns.clone(), towel.clone());
            }
        }
        count
    }


}

#[cached]
fn ways(towel_patterns: Vec<String>,towel: String) -> u64{
    if towel.is_empty(){
        return 1;
    }
    let mut count:u64 = 0;
    for pattern in towel_patterns.iter(){
        if towel.starts_with(pattern){
            let new_towel = towel.replacen(pattern, "", 1);
            count += ways(towel_patterns.clone(), new_towel);
        }
    }
    count
}


impl From<String> for Solver{
    fn from(value: String) -> Self {
        let (top, bottom) = value.split_once("\n\n").unwrap();
        let towel_patterns:Vec<_> = top.split(",").map(|x| x.trim()).collect();
        let towels:Vec<_> = bottom.lines().map(|x| x.trim()).collect();
        Self{
            towel_patterns: towel_patterns.iter().map(|x| x.to_string()).collect(),
            towels: towels.iter().map(|x| x.to_string()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
        // 1555825069 too low
    }
}
