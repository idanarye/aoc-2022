use std::cmp::Reverse;

use hashbrown::HashMap;
use itertools::Itertools;

#[derive(Debug)]
pub enum Instruction {
    CdRoot,
    CdUp,
    Cd(String),
    Dir(Vec<DirItem>),
}

#[derive(Debug, Clone)]
pub enum DirItem {
    File { size: usize, name: String },
    Subdirectory(String),
}

pub fn generator(input: &str) -> Vec<Instruction> {
    let mut result = Vec::new();
    let mut dir_content: Option<Vec<DirItem>> = None;
    for line in input.lines() {
        let mut parts = line.split(" ");
        match parts.next().unwrap() {
            "$" => {
                if let Some(dir_content) = dir_content.take() {
                    result.push(Instruction::Dir(dir_content));
                }
                match parts.next().unwrap() {
                    "cd" => result.push(match parts.next().unwrap() {
                        "/" => Instruction::CdRoot,
                        ".." => Instruction::CdUp,
                        dir => Instruction::Cd(dir.to_owned()),
                    }),
                    "ls" => {
                        dir_content = Some(Vec::new());
                    }
                    unknown_command => panic!("Cannot handle {}", unknown_command),
                }
            }
            "dir" => {
                dir_content
                    .as_mut()
                    .unwrap()
                    .push(DirItem::Subdirectory(parts.next().unwrap().to_owned()));
            }
            size => {
                dir_content.as_mut().unwrap().push(DirItem::File {
                    size: size.parse().unwrap(),
                    name: parts.next().unwrap().to_owned(),
                });
            }
        }
        assert!(parts.next().is_none());
    }
    if let Some(dir_content) = dir_content {
        result.push(Instruction::Dir(dir_content));
    }
    result
}

#[derive(Debug)]
struct Filesystem {
    content: HashMap<Vec<String>, Vec<DirItem>>,
}

impl Filesystem {
    fn build_from_instructions(instructions: &[Instruction]) -> Self {
        let mut filesystem = Self {
            content: Default::default(),
        };
        let mut position = vec![];
        for instruction in instructions {
            match instruction {
                Instruction::CdRoot => {
                    position = vec![];
                }
                Instruction::CdUp => {
                    position.pop();
                }
                Instruction::Cd(dir) => {
                    position.push(dir.clone());
                }
                Instruction::Dir(content) => {
                    filesystem.content.insert(position.clone(), content.clone());
                }
            }
        }
        filesystem
    }
}

#[derive(Debug)]
struct FilesystemDirectorySizes {
    sizes: HashMap<Vec<String>, usize>,
}

impl FilesystemDirectorySizes {
    fn calc_from(filesystem: &Filesystem) -> Self {
        let mut result = Self {
            sizes: HashMap::new(),
        };
        let mut all_directories = filesystem.content.keys().cloned().collect_vec();
        all_directories.sort_by_key(|path| Reverse(path.len()));
        for mut path in all_directories {
            let total_size: usize = filesystem.content[&path]
                .iter()
                .map(|entry| match entry {
                    DirItem::File { size, name: _ } => *size,
                    DirItem::Subdirectory(dirname) => {
                        path.push(dirname.clone());
                        let size = result.sizes[&path];
                        path.pop().unwrap();
                        size
                    }
                })
                .sum();
            result.sizes.insert(path, total_size);
        }
        result
    }
}

pub fn part_1(input: &[Instruction]) -> usize {
    let filesystem = Filesystem::build_from_instructions(input);
    let sizes = FilesystemDirectorySizes::calc_from(&filesystem);
    sizes.sizes.values().filter(|&&size| size <= 100_000).sum()
}

pub fn part_2(input: &[Instruction]) -> usize {
    let filesystem = Filesystem::build_from_instructions(input);
    let sizes = FilesystemDirectorySizes::calc_from(&filesystem);
    let total_disk_capacity = 70_000_000;
    let currently_unused_capacity = total_disk_capacity - sizes.sizes[&vec![]];
    let required_capacity = 30_000_000;
    let need_to_free = required_capacity - currently_unused_capacity;
    sizes
        .sizes
        .values()
        .copied()
        .filter(|&size| need_to_free <= size)
        .min()
        .unwrap()
}
