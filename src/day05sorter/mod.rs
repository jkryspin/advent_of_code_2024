use std::collections::{HashMap, HashSet};

pub struct Sorter{}
impl Sorter {
    pub fn topological_sort(&self, graph: &HashMap<u32, Vec<u32>>) -> Vec<u32> {
        let mut sorted = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_mark = HashSet::new();

        for &node in graph.keys() {
            if !visited.contains(&node) {
                self.visit(node, graph, &mut visited, &mut temp_mark, &mut sorted);
            }
        }

        sorted.reverse();
        sorted
    }

    fn visit(
        &self,
        node: u32,
        graph: &HashMap<u32, Vec<u32>>,
        visited: &mut HashSet<u32>,
        temp_mark: &mut HashSet<u32>,
        sorted: &mut Vec<u32>,
    ) {
        if temp_mark.contains(&node) {
            panic!("Graph has a cycle");
        }

        if !visited.contains(&node) {
            temp_mark.insert(node);
            if let Some(neighbors) = graph.get(&node) {
                for &neighbor in neighbors {
                    self.visit(neighbor, graph, visited, temp_mark, sorted);
                }
            }
            temp_mark.remove(&node);
            visited.insert(node);
            sorted.push(node);
        }
    }
}
