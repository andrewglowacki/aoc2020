use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::HashSet;

fn get_group_yes_count(lines: &mut Lines<BufReader<File>>) -> Option<usize> {
    lines.filter_map(|line| line.ok())
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect::<HashSet<char>>())
        .fold(None::<HashSet::<char>>, |set, answers| {
            if let Some(set) = set {
                Some(set.intersection(&answers)
                    .map(|c| *c)
                    .collect::<HashSet<char>>())
            } else {
                None
            }
        })
        .map(|set| set.len())
}

fn main() {
    let path = Path::new("input.txt");
    let file = File::open(path).unwrap();
    
    let mut lines = BufReader::new(file).lines();
    let mut sums = 0;
    while let Some(group_count) = get_group_yes_count(&mut lines) {
        sums += group_count;
    }

    println!("Group sums is: {}", sums);
}
