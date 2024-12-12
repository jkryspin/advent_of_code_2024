advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i128> {
    // 2333133121414131402
    // 00...111...2...333.44.5555.6666.777.888899
    let mut i = 0;
    let mut output = Vec::new();
    for c in input.chars().map(|c|c.to_digit(10).unwrap()) {
        if i % 2 == 0 {
            // c == length of block
            for _ in 0..c {
                output.push(i/2);
            }
        } else {
            // c == length of free space
            for _ in 0..c {
                output.push(-1);
            }
        }
        i += 1;
    }
    let mut i = 0;
    // get last element not equal to .
    while i < output.len() {
        // println!("{}", output.join(""));
        if output[i] == -1 {
            if let Some(pos) = output.iter_mut().rposition(|x| *x != -1) {
                if pos < i {
                    println!("we are done");
                    // remove all remaining . from output
                    output = output.into_iter().filter(|x| *x != -1).collect();

                    break;
                }
               output[i]= output.remove(pos);
            } else {
                panic!("no more values");
            }
        }
        i += 1;
    }

    let mut sum = 0;
    output.into_iter().enumerate().for_each(|(i, c)| {
        sum += i as i128 * c
    });

    Some(sum)
}

pub fn part_two(input: &str) -> Option<i128> {
    let mut i = 0;
    let mut output = Vec::new();
    for c in input.chars().map(|c|c.to_digit(10).unwrap()) {
        if i % 2 == 0 {
            // c == length of block
            for _ in 0..c {
                output.push(i/2);
            }
        } else {
            // c == length of free space
            for _ in 0..c {
                output.push(-1);
            }
        }
        i += 1;
    }
    let mut i = output.len() - 1;
    while i > 0 {
        print_output(&output);
        if output[i] == -1 {
            i-=1;
            continue;
        }

        let block_len = output.iter().rev().skip(output.len() -i).take_while(|&&x| x == output[i]).count() +1;

        let mut j = 0;
        while j < i {
            if output[j] == -1 {
                let free_space_len = output.iter().skip(j).take_while(|&&x| x == -1).count();
                if free_space_len >= block_len {
                    for k in 0..block_len {
                        output[j + k] = output[i];
                    }
                    for k in 0..block_len {
                        output[i - k] = -1;
                    }
                    break;
                }
            }
            j+=1;
        }
        i = i.saturating_sub(block_len)
    }

    print_output(&output);

    let mut sum = 0;
    output.into_iter().enumerate().filter(|(_, x)|*x!=-1).for_each(|(i, c)| {
        sum += i as i128 * c
    });

    Some(sum)
}
fn print_output(output: &Vec<i128>) {
    return;
    println!("{}", output.iter().map(|x| {
        if *x == -1 {
            return ".".to_string();
        }
        x.to_string()
    }).collect::<Vec<String>>().join(""));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}