use std::collections::{HashSet, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 20)
}

fn solve(input: &str, max_len: usize) -> Option<u32> {
    let solver = Solver::from(input);
    let start = solver.grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .unwrap();
    let end = solver.grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (x, y)))
        .unwrap();
    let costs = bfs(&solver.grid, start, end).unwrap();
    let mut moves_saved = vec![];
    for y in 0..costs.len() {
        for x in 0..costs[0].len() {
            if costs[y][x] != 0 {
                moves_saved.extend(bfs_with_cap(&costs, (x, y), max_len).unwrap());
            }
        }
    }
    moves_saved.sort();
    moves_saved.dedup();
    let costs_calculated = moves_saved.iter().map(|(start, end, steps)| {
        let starting_cost = costs[start.1][start.0];
        let ending_cost = costs[end.1][end.0];
        if starting_cost > ending_cost {
            return 0;
        }
        let steps_saved = ending_cost as i32 - starting_cost as i32 - *steps as i32;
        steps_saved as u32
    }).collect::<Vec<u32>>();

    let count = costs_calculated.iter().filter(|&&x| x >= 100).count();
    Some(count as u32)
}

struct Solver {
    grid: Vec<Vec<char>>
}

fn bfs_with_cap(grid: &Vec<Vec<u32>>, start: (usize, usize), max_len: usize) -> Option<Vec<((usize, usize), (usize, usize), usize)>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, vec![start]));
    visited.insert(start);
    let mut moves_saved = vec![];
    while let Some((pos, steps)) = queue.pop_front() {
        let (x, y) = pos;
        if steps.len() - 1 >= 2 && steps.len() - 1 <= max_len && grid[y][x] != 0 {
            if steps.iter().any(|(x, y)| grid[*y][*x] == 0) {
                moves_saved.push((start, pos, steps.len() - 1));
            }
        }
        if steps.len() - 1 >= 2 && steps.len() - 1 > max_len {
            continue;
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 {
                continue;
            }
            if nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            let mut new_steps = steps.clone();
            new_steps.push((nx, ny));
            queue.push_back(((nx, ny), new_steps));
        }
    }
    Some(moves_saved)
}

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> Option<Vec<Vec<u32>>> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((start, 1));
    visited.insert(start);
    let mut costs = vec![vec![0; grid[0].len()]; grid.len()];
    while let Some((pos, steps)) = queue.pop_front() {
        let (x, y) = pos;
        costs[y][x] = steps;
        if pos == end {
            return Some(costs);
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 {
                continue;
            }
            if nx >= grid[0].len() as i32 || ny >= grid.len() as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[ny][nx] == '#' {
                continue;
            }
            if visited.contains(&(nx, ny)) {
                continue;
            }
            visited.insert((nx, ny));
            queue.push_back(((nx, ny), steps + 1));
        }
    }
    None
}

impl From<&str> for Solver {
    fn from(s: &str) -> Self {
        let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        Self { grid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}