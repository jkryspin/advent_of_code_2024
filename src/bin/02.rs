use std::str::FromStr;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut count = 0;
    lines.iter().for_each(|l|{
        let levels = l.split(" ").map(|i|{
            i32::from_str(i).unwrap()
        }).collect::<Vec<i32>>();

        if is_safe(levels){
            count+=1;
        }
    });

    Some(count)
}

fn is_safe(levels: Vec<i32>) -> bool{
    let mut prev = None;
    let mut is_asc = None;
    for curr in levels.iter() {
        if prev.is_some() {
            if is_asc.is_some(){
                let is_curr_asc = curr - prev.unwrap() > 0;
                if is_curr_asc != is_asc.unwrap() {
                    return false;
                }
            }

            let diff = curr.abs_diff(prev.unwrap());
            if  diff <1 || diff > 3{
                return false;
            }

            if curr - prev.unwrap() > 0{
                is_asc = Some(true);
            }else{
                is_asc = Some(false);
            }
        }

        prev =Some(*curr);
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut count = 0;
    lines.iter().for_each(|l|{
        let levels = l.split(" ").map(|i|{
            i32::from_str(i).unwrap()
        }).collect::<Vec<i32>>();

        if problem_retrier(levels){
            count+=1;
        }

    });

    Some(count)
}

fn problem_retrier(levels: Vec<i32>) -> bool{
   if is_safe(levels.clone()){
       return true;
   }
    // iterate is_safe attempts with each postion removed
    for i in 0..levels.len(){
        let mut new_levels = levels.clone();
        new_levels.remove(i);
        if is_safe(new_levels){
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
