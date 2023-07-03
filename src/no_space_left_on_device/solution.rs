use std::{cell::RefCell, rc::Rc};

const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    println!("In No Space Left_on_device solver");
    solve_part_one(&INPUT);
    solve_part_two(&INPUT);
}

#[derive(Clone, Debug)]
struct File<'s> {
    name: &'s str,
    size: u32,
}
impl<'s> File<'s> {
    pub fn new(name: &str, size: u32) -> File {
        return File {
            name: name,
            size: size,
        };
    }
}
#[derive(Clone, Debug)]
struct Directory<'s> {
    name: &'s str,
    files: Vec<File<'s>>,
    sub_dirs: Vec<usize>,
    parent_dir: Option<usize>,
}

impl<'s> Directory<'s> {
    pub fn new(name: &str) -> Directory {
        return Directory {
            name,
            files: vec![],
            sub_dirs: vec![],
            parent_dir: None,
        };
    }

    pub fn add_file(&mut self, file: File<'s>) {
        self.files.push(file);
    }
}

struct FileSystem<'s> {
    directories: Rc<RefCell<Vec<Rc<RefCell<Directory<'s>>>>>>,
    current_dir: Rc<RefCell<usize>>,
}

impl<'s> FileSystem<'s> {
    fn new() -> FileSystem<'s> {
        return FileSystem {
            directories: Rc::new(RefCell::new(Vec::new())),
            current_dir: Rc::new(RefCell::new(0)),
        };
    }
}

enum NextAction {
    ReadDirContents,
    ChangeDir,
    None,
}

fn solve_part_one(contents: &str) {
    let file_system = Rc::new(RefCell::new(FileSystem::new()));
    let root = Directory::new("/");
    file_system
        .borrow_mut()
        .directories
        .borrow_mut()
        .push(Rc::new(RefCell::new(root)));
    let mut current_mode = NextAction::None;
    for line in contents.lines().skip(1) {
        match current_mode {
            NextAction::None => {
                if line.starts_with("ls") {
                    current_mode = NextAction::ReadDirContents;
                } else if line.starts_with("cd") {
                    current_mode = NextAction::ChangeDir;
                } else {
                }
            }
            NextAction::ChangeDir => {
                let dir_name = line
                    .split("cd ")
                    .nth(0)
                    .unwrap_or_else(|| panic!("No argument provided for cd"));
                match dir_name {
                    ".." => {
                        let fs_clone = Rc::clone(&file_system);
                        let current_dir_clone =
                            Rc::clone(&fs_clone.borrow().current_dir);
                        let fs_clone = Rc::clone(&file_system);
                        let parent_dir_option =
                            fs_clone.borrow_mut().directories.borrow_mut()
                                [*current_dir_clone.borrow()]
                            .borrow()
                            .parent_dir;
                        if parent_dir_option.is_some() {
                            *current_dir_clone.borrow_mut() =
                                parent_dir_option.unwrap();
                        }
                    }
                    "" => panic!("Not a valid argument"),
                    dir => {
                        let fs_clone = Rc::clone(&file_system);
                        let fs_clone_borrowed = fs_clone.borrow();
                        let current_dir_clone =
                            Rc::clone(&fs_clone_borrowed.current_dir);
                        let current_dirs =
                            fs_clone_borrowed.directories.borrow();
                        let current_dir =
                            current_dirs[*current_dir_clone.borrow()].borrow();
                        let sub_dirs = &current_dir.sub_dirs;
                        let mut found_dir = false;
                        for sub_dir in sub_dirs {
                            let sub_dir_obj = current_dirs[*sub_dir].borrow();
                            if sub_dir_obj.name == dir {
                                *current_dir_clone.borrow_mut() = *sub_dir;
                                found_dir = true;
                                break;
                            }
                        }
                        if !found_dir {
                            println!("No such directory");
                        }
                    }
                }
            }
            NextAction::ReadDirContents => {
                let mut values_iter = line.split(" ").take(2);

                let values = (values_iter.next(), values_iter.next());
                match values {
                    (Some("dir"), Some(dir_name)) => {
                        let new_dir = Directory::new(dir_name);
                        file_system
                            .borrow_mut()
                            .directories
                            .borrow_mut()
                            .push(Rc::new(RefCell::new(new_dir)));
                        let current_dir_id_clone =
                            Rc::clone(&file_system.borrow().current_dir);
                        file_system.borrow().directories.borrow()
                            [*current_dir_id_clone.borrow()]
                        .borrow_mut()
                        .sub_dirs
                        .push(
                            file_system.borrow().directories.borrow().len() - 1,
                        );
                    }
                    (Some(size), Some(file_name)) => {
                        let file_size = size.parse::<u32>().unwrap();
                        let file = File::new(file_name, file_size);
                        let current_dir_id_clone =
                            Rc::clone(&file_system.borrow().current_dir);
                        file_system.borrow().directories.borrow()
                            [*current_dir_id_clone.borrow()]
                        .borrow_mut()
                        .add_file(file);
                    }
                    _ => panic!("Unknown command"),
                }
            }
        }
    }
    let fs_clone = Rc::clone(&file_system);
    println!("{:?}", fs_clone.borrow().directories.borrow());
    // print_directories_with_files(&file_system, 0);
    println!("Solution part 1: {}", 3);
}

// fn print_directories_with_files(fs: &FileSystem, depth: u32) {
//     let current_dirs_clone = Rc::clone(&fs.directories);
//     let current_dir_clone = Rc::clone(&fs.current_dir);
//     let current_dir = &current_dirs_clone.borrow()[*current_dir_clone.borrow()]
//         as &Rc<RefCell<Directory<'_>>>;
//     for file in &current_dir.borrow().files {
//         println!("{}{} {}", " ".repeat(depth as usize), file.size, file.name);
//     }
//     for sub_dir in &current_dir.borrow().sub_dirs {
//         let sub_dir_obj = &current_dirs_clone.borrow()[*sub_dir]
//             as &Rc<RefCell<Directory<'_>>>;
//         println!(
//             "{}{}",
//             " ".repeat(depth as usize),
//             sub_dir_obj.borrow().name
//         );
//         *current_dir_clone.borrow_mut() = *sub_dir;
//         print_directories_with_files(fs, depth + 1);
//     }
// }

fn solve_part_two(contents: &str) {
    println!("Solution part 2: {}", 3);
}
