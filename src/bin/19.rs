use std::collections::VecDeque;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let solver = Solver::from(input.to_string());
    println!("{:?}", solver);
    Some(solver.solve())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[derive(Debug)]
struct Solver{
    towel_patterns: Vec<String>,
    towels: Vec<String>,
}

impl Solver{
    fn solve(&self) -> u32{
        let mut count = 0;
        for towel in &self.towels{
            if self.is_possible(towel){
                count += 1;
            }
        }

        count
    }

    fn is_possible(&self, towel: &str) -> bool{
        let mut q = VecDeque::new();
        q.push_back(towel.to_string());
        while let Some(current) = q.pop_front(){
            if current.is_empty(){
                return true;
            }
            for pattern in &self.towel_patterns{
                if current.starts_with(pattern){
                    let new_towel = current.replace(pattern, "");
                    q.push_back(new_towel);
                }
            }
        }
        false
    }
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
        assert_eq!(result, None);
    }
}
