use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    path::{Path, PathBuf},
};

type FileSystem = HashMap<PathBuf, Directory>;

#[derive(Debug, Default)]
struct Directory {
    files: HashMap<String, usize>,
    directories: HashSet<PathBuf>,
    total_size: usize,
}

fn build_file_system(input: &str) -> FileSystem {
    let mut file_system = FileSystem::new();

    let mut path = PathBuf::new();
    file_system.insert(path.clone(), Default::default());
    for line in input.lines() {
        if let Some(arg) = line.strip_prefix("$ cd ") {
            path = match arg {
                "/" => PathBuf::from(""),
                ".." => path.parent().unwrap().to_path_buf(),
                subdir => path.join(subdir),
            };

            if !file_system.contains_key(&path) {
                file_system.insert(path.clone(), Default::default());
            }
        } else if line.starts_with("$ ls") {
            // Nothing to do
        } else if let Some(subdir) = line.strip_prefix("dir ") {
            let subdir = path.join(subdir);
            file_system
                .get_mut(&path)
                .unwrap()
                .directories
                .insert(subdir);
        } else {
            let (size, name) = line.split(' ').collect_tuple::<(&str, &str)>().unwrap();
            file_system
                .get_mut(&path)
                .unwrap()
                .files
                .insert(name.to_owned(), size.parse::<usize>().unwrap());
        }
    }

    file_system
}

fn compute_directory_size(file_system: &mut FileSystem, path: &Path) {
    let mut total_size = 0;

    let directories = file_system[path].directories.clone();
    for directory in &directories {
        compute_directory_size(file_system, directory);
        total_size += file_system[directory].total_size;
    }

    for file in &file_system[path].files {
        total_size += file.1;
    }

    file_system.get_mut(path).unwrap().total_size = total_size;
}

fn compute_directory_size_100000_sum(file_system: &FileSystem, path: &Path) -> usize {
    let mut total_size = if file_system[path].total_size < 100000 {
        file_system[path].total_size
    } else {
        0
    };

    for directory in &file_system[path].directories {
        total_size += compute_directory_size_100000_sum(file_system, directory);
    }

    total_size
}

fn preprocess(input: &str) -> FileSystem {
    let mut filesystem = build_file_system(input);
    compute_directory_size(&mut filesystem, &PathBuf::new());
    filesystem
}

fn part1(input: &str) -> String {
    let filesystem = preprocess(input);
    compute_directory_size_100000_sum(&filesystem, &PathBuf::new()).to_string()
}

fn part2(input: &str) -> String {
    let filesystem = preprocess(input);

    let free = 70000000 - filesystem[&PathBuf::new()].total_size;
    let delete_size = 30000000 - free;
    let mut best_dir_size = usize::MAX;
    for directory in filesystem.values() {
        if directory.total_size > delete_size {
            best_dir_size = best_dir_size.min(directory.total_size);
        }
    }
    best_dir_size.to_string()
}

crate::run!();

crate::test_example_aoc!(95437, 24933642);

crate::test_aoc!(1543140, 1117448);
