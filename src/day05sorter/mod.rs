use std::collections::{HashMap, HashSet};
use std::hash::Hash;

pub struct Sorter<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Eq + Hash + Copy> Sorter<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn topological_sort(&self, graph: &HashMap<T, Vec<T>>) -> Vec<T> {
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
        node: T,
        graph: &HashMap<T, Vec<T>>,
        visited: &mut HashSet<T>,
        temp_mark: &mut HashSet<T>,
        sorted: &mut Vec<T>,
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
