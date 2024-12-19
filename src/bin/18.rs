use std::collections::VecDeque;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let res = solve(input, true, 71, 71, 1024);
    Some(res as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let grid: Vec<Vec<char>> = vec![vec!['.'; 71]; 71];
    let mut positions: Vec<(usize, usize, usize)> = Vec::new();
    input.lines().enumerate().for_each(|(i, s)| {
        let (left, right) = s.split_once(',').unwrap();
        positions.push((left.parse().unwrap(), right.parse().unwrap(), i));
    });

    let mut i = 1024usize;
    loop {
        let mut temp_grid = grid.clone();
        let mut last = None;
        for (x, y, curr_i) in positions.iter() {
            if curr_i >= &i {
                break;
            }
            last = Some((x, y));
            temp_grid[*y][*x] = '#';
        }
        let res = bfs(&temp_grid, (0, 0), (71 - 1, 71 - 1));
        if res == 0 {
            return Some(format!("{},{}", last.unwrap().0, last.unwrap().1));
        }
        i += 1;
    }
}

fn solve(input: &str, part_one: bool, width: usize, height: usize, limit: usize) -> i32 {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; width]; height];
    let mut positions: Vec<(usize, usize, usize)> = Vec::new();
    input.lines().enumerate().for_each(|(i, s)| {
        let (left, right) = s.split_once(',').unwrap();
        positions.push((left.parse().unwrap(), right.parse().unwrap(), i));
    });

    for (x, y, i) in positions.iter() {
        if part_one && i >= &limit {
            break;
        }
        grid[*y][*x] = '#';
    }

    
    bfs(&grid, (0, 0), (width - 1, height - 1))
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut q = VecDeque::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    q.push_back((start, 0));
    while let Some((curr, len)) = q.pop_front() {
        if visited[curr.1][curr.0] {
            continue;
        }
        visited[curr.1][curr.0] = true;
        if curr == end {
            return len;
        }

        let dirs = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dx, dy) in dirs {
            let x = curr.0 as i32 + dx;
            let y = curr.1 as i32 + dy;
            if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                continue;
            }
            if grid[y as usize][x as usize] == '#' {
                continue;
            }
            q.push_back(((x as usize, y as usize), len + 1));
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
