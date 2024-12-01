use std::collections::{HashMap, HashSet};

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut lefts = vec![];
    let mut rights = vec![];
    lines.iter().for_each(|l|{
            let mut ans = l.split_whitespace();
        lefts.push(ans.next().unwrap().parse::<i32>().unwrap());
        rights.push(ans.next().unwrap().parse::<i32>().unwrap());
        assert!(ans.next().is_none())
    });

    lefts.sort();
    rights.sort();

    let mut diffs =0;
    for(left, right) in lefts.iter().zip(rights) {
        diffs += (right - left).abs();
    }
    Some(diffs)
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut lefts = vec![];
    let mut rights = vec![];
    lines.iter().for_each(|l|{
        let mut ans = l.split_whitespace();
        lefts.push(ans.next().unwrap().parse::<i32>().unwrap());
        rights.push(ans.next().unwrap().parse::<i32>().unwrap());
        assert!(ans.next().is_none())
    });

    let mut num_found = HashMap::new();
    rights.iter().for_each(|item|{
        if num_found.contains_key(item) {
            num_found.insert(item, num_found[item] + 1);
        }else{
            num_found.insert(item, 1);
        }
    });

    let mut diffs =0;
    lefts.iter().for_each(|left|{
        diffs += left * num_found.get(left).unwrap_or(&0);
    });
    Some(diffs)

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
