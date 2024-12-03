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

                count += (a * b);
            }
     }
    Some(count)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut count = 0;
    let mut enabled_ranges = Vec::new();
    let mut disabled_ranges = Vec::new();
        let re = regex::Regex::new(r"do()").unwrap();
        let matches = re.captures_iter(input);
        matches.for_each(|m|{
            // insert the end of the match into the hashmap as another vector item
            enabled_ranges.push(m.get(0).unwrap().end());
        });

        let re2 = regex::Regex::new(r"don't()").unwrap();
        let matches = re2.captures_iter(input);
        matches.for_each(|m|{
            disabled_ranges.push(m.get(0).unwrap().end());
        });
        enabled_ranges.sort();
        disabled_ranges.sort();

        // get each group in this regex
        let re = regex::Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
        let matches = re.captures_iter(input);
        for cap in matches {
            let cap_iter = cap.iter();
            let mut cap_iter = cap_iter;
            let loc = cap_iter.next().unwrap().unwrap().start();
            let a = cap_iter.next().unwrap().unwrap().as_str().parse::<i32>().unwrap();
            let b = cap_iter.next().unwrap().unwrap().as_str().parse::<i32>().unwrap();

            let mut enabled_range = 0;
            for e in enabled_ranges.iter(){
                if e < &loc{
                    enabled_range = *e as i32;
                }
            }
            let mut disabled_range = 0;
            for d in disabled_ranges.iter(){
                if d < &loc{
                    disabled_range = *d as i32;
                }
            }
            if enabled_range >= disabled_range{
                // print enabled range and disabled range
                // println!("enabled range: {}, disabled range: {}", enabled_range, disabled_range);
                count += a * b;
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
