use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
advent_of_code::solution!(16);

fn shortest_path_with_cost(grid: &Vec<Vec<char>>, start: (usize, usize, Direction, Direction), end: (usize, usize)) -> Option<(usize, Vec<(usize, usize)>)> {
    let mut dist: HashMap<(usize, usize, Direction, Direction), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut predecessors: HashMap<(usize, usize, Direction, Direction), (usize, usize, Direction, Direction)> = HashMap::new();

    // Initialize the distance to the start node to 0
    dist.insert(start, 0);
    heap.push(State { cost: 0, position: start });

    // Directions for moving in the grid (up, down, left, right)
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some(State { cost, position }) = heap.pop() {
        // If we reached the end, reconstruct the path and return the cost and path
        if position.0 == end.0 && position.1 == end.1 {
            let mut path = vec![(position.0, position.1)];
            let mut current = position;
            while let Some(&pred) = predecessors.get(&current) {
                path.push((pred.0, pred.1));
                current = pred;
            }
            path.reverse();
            return Some((cost, path));
        }

        // If the cost is greater than the recorded distance, skip this node
        if cost > *dist.get(&position).unwrap_or(&usize::MAX) {
            continue;
        }

        // Explore the neighbors
        for direction in &directions {
            let new_position = (
                (position.0 as isize + direction.0) as usize,
                (position.1 as isize + direction.1) as usize,
                Direction::from(direction),
                position.2,
            );

            // Check if the new position is within the grid bounds
            if new_position.0 < grid.len() && new_position.1 < grid[0].len() {
                if grid[new_position.0][new_position.1] == '#' {
                    continue;
                }
                // d is current direction
                let next_cost = if Direction::from(direction) != position.2 {
                    cost + 1001
                } else {
                    cost + 1
                };

                // If the new cost is less than the recorded distance, update the distance and push to the heap
                if next_cost < *dist.get(&new_position).unwrap_or(&usize::MAX) {
                    dist.insert(new_position, next_cost);
                    heap.push(State { cost: next_cost, position: new_position });
                    predecessors.insert(new_position, position);
                }
            }
        }
    }

    // If we reach here, there is no path to the end
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .unwrap();
    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (x, y)))
        .unwrap();

    let start = (start.0, start.1, Direction::Right, Direction::Right);

    let asn = shortest_path_with_cost(&grid, start, end).map(|(cost, path)| (1000 + cost as u32, path));

    return asn.map(|(cost, path)| cost);

}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let start = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'S').map(|x| (x, y)))
        .unwrap();
    let end = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| row.iter().position(|&c| c == 'E').map(|x| (x, y)))
        .unwrap();

    let start = (start.0, start.1, Direction::Right, Direction::Right);

    let (cost, positions)= shortest_path_with_cost(&grid, start, end).map(|(cost, path)| (1000 + cost as u32, path)).unwrap();
    println!("{:?}", positions);
    Some(positions.len() as u32)
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize, Direction, Direction), // (x, y, current direction, previous direction)
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<&(isize, isize)> for Direction {
    fn from((x, y): &(isize, isize)) -> Self {
        match (x, y) {
            (0, 1) => Direction::Down,
            (1, 0) => Direction::Right,
            (0, -1) => Direction::Up,
            (-1, 0) => Direction::Left,
            _ => panic!("Invalid direction"),
        }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Edge {
    node: (usize, usize),
    cost: usize,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one(r#"#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"#);
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
