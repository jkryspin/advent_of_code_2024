

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut count = 0;
     for line in input.lines() {
         // get each group in this regex
            let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
            let matches = re.captures_iter(line);
            for cap in matches {
                let cap_iter = cap.iter();
                let mut cap_iter = cap_iter.skip(1);
                let a = cap_iter.next().unwrap().unwrap().as_str().parse::<u32>().unwrap();
                let b = cap_iter.next().unwrap().unwrap().as_str().parse::<u32>().unwrap();

                count += a * b;
            }
     }
    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut count = 0;
    let re = regex::Regex::new(r"mul\((?P<a>\d+),(?P<b>\d+)\)|(?P<do>do\(\))|(?P<dont>don't\(\))").unwrap();
    let mut enabled = true;

    for cap in re.captures_iter(input) {
        match cap.name("do") {
            Some(_) => enabled = true,
            _ => {}
        }

        match cap.name("dont") {
            Some(_) => enabled = false,
            _ => {}
        }

        if enabled {
            match (cap.name("a"), cap.name("b")) {
                (Some(a), Some(b)) => {
                    count += a.as_str().parse::<i32>().unwrap() * b.as_str().parse::<i32>().unwrap();
                }
                _ => {}
            }
        }
    }
    Some(count)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
