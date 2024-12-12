use std::collections::{HashMap, HashSet};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

fn solve(input: &str, part: bool) -> u32 {
    let grid = input.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let regions = get_regions(&grid);
    let mut total = 0;

    for ((cell, _start_x, _start_y), region) in regions.iter() {
        let mut perimeter = 0;
        for (x, y) in region.iter() {
            let mut sides = 4;
            for (nx, ny) in find_neighbors(*x as usize, *y as usize, &grid) {
                if !region.contains(&(nx as u32, ny as u32)) {
                    perimeter += 1;
                }
                sides -= 1;
            }
            perimeter += sides;
        }
        println!("Region: {:?}", region);
        let region_i32 = region.iter().map(|(x, y)| (*x as i32, *y as i32)).collect::<Vec<_>>();
        let sides = calculate_region_sides(&region_i32);
        if part {
            total += region.len() as u32 * perimeter as u32;
        } else {
            println!("Region of {} plants with price {} * {}", cell, region.len(), sides);
            total += region.len() as u32 * sides as u32;
        }
    }

    total
}

fn get_regions(grid: &Vec<Vec<char>>) -> HashMap<(char, usize, usize), Vec<(u32, u32)>> {
    let mut regions: HashMap<(char, usize, usize), Vec<(u32, u32)>> = HashMap::new();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if visited[y][x] {
                continue;
            }
            let mut stack = vec![(x, y)];
            let mut region = Vec::new();
            while let Some((x, y)) = stack.pop() {
                if region.contains(&(x as u32, y as u32)) {
                    continue;
                }
                visited[y][x] = true;
                region.push((x as u32, y as u32));
                for (nx, ny) in find_neighbors(x, y, &grid) {
                    if grid[ny][nx] == *cell {
                        stack.push((nx, ny));
                    }
                }
            }
            for (x_r, y_r) in region.iter() {
                regions.entry((*cell, x, y)).or_insert(Vec::new()).push((*x_r, *y_r));
            }
        }
    }

    regions
}

fn calculate_region_sides(region: &Vec<(i32, i32)>) -> u32 {
    println!("Region: {:?}", region);
    let mut perimeter = HashSet::new();
    for (x, y) in region.iter() {
        for (nx, ny) in find_neighbors_no_bounds(*x, *y) {
            if !region.contains(&(nx, ny)) {
                perimeter.insert((nx, ny));
            }
        }
    }
    // println!("Perimeter: {:?}", perimeter);
    let mut print_grid = vec![vec!['.'; 10]; 10];
    for (x, y) in perimeter.iter() {
        if *x >= 0 && *y >= 0 && (*x as usize) < 10 && (*y as usize) < 10 {
            print_grid[*y as usize][*x as usize] = '#';
        }
    }

    // Print the grid
    for row in print_grid {
        // println!("{}", row.iter().collect::<String>());
    }
    // print perimeter in grid, with . for empty, # for full
    // get number of connected perimeter cells
    let mut connected = 0;
    while !perimeter.is_empty() {
        let mut stack = vec![perimeter.iter().next().unwrap().clone()];
        perimeter.remove(&stack[0]);
        while let Some((x, y)) = stack.pop() {
            for (nx, ny) in find_neighbors_no_bounds(x, y) {
                if perimeter.contains(&(nx, ny)) {
                    perimeter.remove(&(nx, ny));
                    stack.push((nx, ny));
                }
            }
        }
        connected += 1;
    }
    connected

}
fn find_neighbors_no_bounds(x: i32, y: i32) -> Vec<(i32, i32)> {
    vec![(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
}

fn find_neighbors(x: usize, y: usize, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if x < grid[0].len() - 1 {
        neighbors.push((x + 1, y));
    }
    if y < grid.len() - 1 {
        neighbors.push((x, y + 1));
    }
    neighbors
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }

    #[test]
    fn test_part_two_a() {
        let result = part_two("AAAA\nBBCD\nBBCC\nEEEC");
        assert_eq!(result, Some(80));
    }
}