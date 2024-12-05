use std::collections::{HashMap, HashSet};
use advent_of_code::day05sorter;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let printer = Printer::new(input);
    let (ordered_pages, _) = printer.get_ordered_pages();

    let middle_pages = ordered_pages
        .iter()
        .map(|page_producer| page_producer.get_middle_page())
        .collect::<Vec<_>>();

    Some(middle_pages.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let printer = Printer::new(input);
    let (_, unordered_pages) = printer.get_ordered_pages();

    let mut sum = 0;
    unordered_pages.iter().for_each(|page_producer| {
        let mut rules = vec![];
        let unique_pages = page_producer.pages.iter().collect::<HashSet<_>>();
        for rule in printer.page_ordering_rules.iter() {
            if unique_pages.contains(&rule.first) && unique_pages.contains(&rule.second) {
                rules.push(rule)
            }
        }

        let mut graph = HashMap::new();
        for rule in rules.iter() {
            graph.entry(rule.first).or_insert(vec![]).push(rule.second);
        }

        let sorter = day05sorter::Sorter {};
        let sorted_pages = sorter.topological_sort(&graph);

        sum += sorted_pages[sorted_pages.len() / 2];
    });

    Some(sum)
}

#[derive(Debug)]
struct Printer {
    page_ordering_rules: Vec<PageOrderingRule>,
    pages_to_produce: Vec<PageProducer>,
}

impl Printer {
    fn get_ordered_pages(&self) -> (Vec<&PageProducer>, Vec<&PageProducer>) {
        let mut ordered_pages = Vec::new();
        let mut unordered_pages = Vec::new();
        for pageProducer in &self.pages_to_produce {
            let mut rules = vec![];
            let unique_pages = pageProducer.pages.iter().collect::<HashSet<_>>();
            for rule in self.page_ordering_rules.iter() {
                if unique_pages.contains(&rule.first) && unique_pages.contains(&rule.second) {
                    rules.push(rule)
                }
            }

            let mut good_rule = true;
            for rule in rules.iter() {
                // println!("{:?}", rule);
                if pageProducer
                    .pages
                    .iter()
                    .position(|page| *page == rule.first)
                    .unwrap()
                    > pageProducer
                        .pages
                        .iter()
                        .position(|page| *page == rule.second)
                        .unwrap()
                {
                    // println!("Not in order");
                    good_rule = false;
                    break;
                }
            }
            if good_rule {
                ordered_pages.push(pageProducer);
            } else {
                unordered_pages.push(pageProducer);
            }
        }
        (ordered_pages, unordered_pages)
    }
    fn new(input: &str) -> Self {
        let (page_ordering_rules_s, pages_to_produce_s) = input.split_once("\n\n").unwrap();
        let page_ordering_rules = page_ordering_rules_s
            .lines()
            .map(|line| {
                let (first, second) = line.split_once("|").unwrap();
                PageOrderingRule {
                    first: first.parse().unwrap(),
                    second: second.parse().unwrap(),
                }
            })
            .collect();
        let pages_to_produce = pages_to_produce_s
            .lines()
            .map(|line| PageProducer {
                pages: line.split(",").map(|page| page.parse().unwrap()).collect(),
            })
            .collect();
        Self {
            page_ordering_rules,
            pages_to_produce,
        }
    }
}

#[derive(Debug)]
struct PageOrderingRule {
    first: u32,
    second: u32,
}

#[derive(Debug)]
struct PageProducer {
    pages: Vec<u32>,
}

impl PageProducer {
    fn get_middle_page(&self) -> u32 {
        let pages = self.pages.clone();
        pages[pages.len() / 2]
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
