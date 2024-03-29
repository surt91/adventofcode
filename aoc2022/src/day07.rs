use std::str::FromStr;

use aoc2021::{data_str, utils::{split_lines, AdventError}};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug)]
enum DirEntry {
    File(usize),
    Dir(String),
}

#[derive(Debug)]
struct FileHierarchy {
    flat_mapping: FxHashMap<String, Vec<DirEntry>>,
}

impl FileHierarchy {
    fn size_of_dir(&self, abs_path: &str) -> usize {
        self.flat_mapping
            .get(abs_path)
            .unwrap()
            .iter()
            .map(|entry|
                match entry {
                    DirEntry::File(size) => *size,
                    DirEntry::Dir(abs_path) => self.size_of_dir(abs_path),
                }
            )
            .sum()
    }

    fn size_of_dirs(&self) -> Vec<usize> {
        self.flat_mapping
            .keys()
            .map(|abs_path| self.size_of_dir(abs_path))
            .collect()
    }
}

impl FromStr for FileHierarchy {
    type Err = AdventError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = split_lines(s);

        let mut pwd: Vec<String> = Vec::new();
        let mut files: FxHashMap<String, Vec<DirEntry>> = FxHashMap::default();
        files.insert("/".to_string(), Vec::new());

        for line in lines {
            if line.starts_with("$ cd ..") {
                pwd.pop();
            } else if line.starts_with("$ cd /") {
                pwd.clear();
                pwd.push("/".to_string());
            } else if line.starts_with("$ cd") {
                let new_dir = line.split(' ')
                    .last()
                    .ok_or(AdventError::WrongNumberOfElements)?;
                pwd.push(new_dir.to_string());
                files.insert(pwd.join("/"), Vec::new());
            } else if line.starts_with("$ ls") {
                // nothing to do
            } else if line.starts_with("dir") {
                let name = line.split(' ')
                    .last()
                    .ok_or(AdventError::WrongNumberOfElements)?;
                pwd.push(name.to_string());
                let abs_path = pwd.join("/");
                pwd.pop();
                files.entry(pwd.join("/")).and_modify(|entries| entries.push(DirEntry::Dir(abs_path)));
            } else {
                let mut iter = line.split(' ');
                let size: usize = iter.next()
                    .ok_or(AdventError::WrongNumberOfElements)?
                    .parse()
                    .map_err(AdventError::Parser)?;
                let _filename = iter.next()
                    .ok_or(AdventError::WrongNumberOfElements)?;
                files.entry(pwd.join("/")).and_modify(|entries| entries.push(DirEntry::File(size)));
            }
        }

        Ok(FileHierarchy {
            flat_mapping: files
        })
    }
}

pub fn run() -> (usize, usize) {

    let input = data_str!("day07");
    let hierarchy: FileHierarchy = input.parse().unwrap();

    (
        size_of_small_dirs(&hierarchy),
        find_smallest_dir_to_delete(&hierarchy)
    )
}

fn size_of_small_dirs(hierarchy: &FileHierarchy) -> usize {
    hierarchy.size_of_dirs().iter()
        .filter(|&&size| size < 100000)
        .sum()
}

fn find_smallest_dir_to_delete(hierarchy: &FileHierarchy) -> usize {
    let total = 70000000;
    let required = 30000000;
    let full = hierarchy.size_of_dir("/");
    let to_delete = required - (total - full);

    *hierarchy.size_of_dirs().iter()
        .filter(|&&size| size > to_delete)
        .sorted()
        .next()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"
            $ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k
        ";

        let hierarchy: FileHierarchy = input.parse().unwrap();

        assert_eq!(size_of_small_dirs(&hierarchy), 95437);
        assert_eq!(find_smallest_dir_to_delete(&hierarchy), 24933642);
    }
}