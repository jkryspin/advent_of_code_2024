advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u128>
{
    solve(input, 25)
}

fn solve(input:&str, iterations:usize) -> Option<u128> {
    let mut stones = input.split(" ").map(|s| s.parse::<u128>().unwrap()).collect::<Vec<_>>();
    let mut sum =0;
    for stone in stones.iter(){
        sum += s(*stone, iterations);
    }
    Some(sum)
}

#[cached::proc_macro::cached]
fn s(stone:u128, iterations:usize) -> u128{
    if iterations == 0{
        return 1;
    }
    if stone == 0{
        return s(1, iterations - 1);
    }
    if stone.to_string().chars().count() % 2 == 0 {
        let stone_string = stone.to_string();
        let (left, right) = stone_string.split_at(stone_string.len() / 2);
        return s(left.parse::<u128>().unwrap(), iterations - 1) + s(right.parse::<u128>().unwrap(), iterations - 1);
    }
    s(stone * 2024, iterations - 1)
}

pub fn part_two(input: &str) -> Option<u128> {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
