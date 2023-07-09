use std::{cell::RefCell, rc::Rc};

const MAX_FS_SIZE: u32 = 70000000;
const NEEDED_FOR_UPDATE: u32 = 30000000;

const INPUT: &str = include_str!("input.txt");

pub fn solve() {
    println!("In No Space Left_on_device solver");
    solve_part_one(&INPUT);
    solve_part_two(&INPUT);
}

#[derive(Clone, Debug)]
struct File {
    size: u32,
}
impl File {
    pub fn new(size: u32) -> File {
        return File { size: size };
    }
}
#[derive(Clone, Debug)]
struct Directory<'s> {
    name: &'s str,
    files: Rc<RefCell<Vec<File>>>,
    sub_dirs: Rc<RefCell<Vec<usize>>>,
    parent_dir: Rc<RefCell<Option<usize>>>,
}

impl<'s> Directory<'s> {
    pub fn new(name: &str) -> Directory {
        return Directory {
            name,
            files: Rc::new(RefCell::new(Vec::new())),
            sub_dirs: Rc::new(RefCell::new(Vec::new())),
            parent_dir: Rc::new(RefCell::new(None)),
        };
    }

    pub fn get_size(&self, fs: &FileSystem) -> u32 {
        let mut size = 0;
        for file in self.files.borrow().iter() {
            size += file.size;
        }

        if self.sub_dirs.borrow().len() == 0 {
            return size;
        }

        for sub_dir in self.sub_dirs.borrow().iter() {
            size += fs.get_dir(*sub_dir).borrow().get_size(fs);
        }

        return size;
    }

    pub fn add_file(&mut self, file: File) {
        self.files.borrow_mut().push(file);
    }

    pub fn add_dir(&mut self, dir: usize) {
        self.sub_dirs.borrow_mut().push(dir);
    }

    pub fn set_parent_dir(&mut self, parent_dir: usize) {
        self.parent_dir = Rc::new(RefCell::new(Some(parent_dir)));
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

    pub fn add_dir(&mut self, dir: Directory<'s>) {
        self.directories
            .borrow_mut()
            .push(Rc::new(RefCell::new(dir)));
    }

    pub fn get_dir(&self, dir: usize) -> Rc<RefCell<Directory<'s>>> {
        return Rc::clone(&self.directories.borrow()[dir]);
    }

    pub fn get_current_dir(&self) -> Rc<RefCell<usize>> {
        return Rc::clone(&self.current_dir);
    }

    pub fn set_current_dir(&mut self, dir: usize) {
        self.current_dir = Rc::new(RefCell::new(dir));
    }
}

fn solve_part_one(contents: &str) -> Rc<RefCell<FileSystem>> {
    let mut file_system = FileSystem::new();
    let root = Directory::new("/");
    file_system.add_dir(root);
    file_system.set_current_dir(0);

    for (idx, line) in contents.lines().enumerate().skip(1) {
        let mut line_tokens = line.split_whitespace();
        let first = line_tokens.next();
        let second = line_tokens.next();
        let third = line_tokens.next();
        match (first, second, third) {
            (_, Some("cd"), Some(dir_name)) => {
                if dir_name == ".." {
                    let current_dirs = &file_system.directories;
                    let current_dir = &file_system.current_dir;
                    let current_dir_idx = *current_dir.borrow();
                    let parent_dir_idx = current_dirs.borrow()[current_dir_idx]
                        .borrow()
                        .parent_dir
                        .borrow()
                        .unwrap();
                    file_system.set_current_dir(parent_dir_idx);
                } else {
                    let current_dirs_clone =
                        Rc::clone(&file_system.directories);
                    let current_dir_clone = Rc::clone(&file_system.current_dir);
                    let current_dir = &current_dirs_clone.borrow()
                        [*current_dir_clone.borrow()]
                        as &Rc<RefCell<Directory<'_>>>;
                    let sub_dirs = &current_dir.borrow().sub_dirs;
                    for sub_dir in sub_dirs.borrow().iter() {
                        let sub_dir_obj = &current_dirs_clone.borrow()[*sub_dir]
                            as &Rc<RefCell<Directory<'_>>>;
                        if sub_dir_obj.borrow().name == dir_name {
                            file_system.set_current_dir(*sub_dir);
                            break;
                        }
                    }
                }
            }
            (_, Some("ls"), None) => {}
            (Some("dir"), Some(dir_name), _) => {
                file_system.add_dir(Directory::new(dir_name));
                let dir_idx = file_system.directories.borrow().len() - 1;
                let current_dir_idx = *file_system.get_current_dir().borrow();
                let current_dir = file_system.get_dir(current_dir_idx);
                current_dir.borrow_mut().add_dir(dir_idx);
                let new_dir = file_system.get_dir(dir_idx);
                new_dir.borrow_mut().set_parent_dir(current_dir_idx);
            }
            (Some(file_size), _, _) => {
                let parse_result = file_size.parse::<u32>();
                if parse_result.is_ok() {
                    let current_dir_idx =
                        *file_system.get_current_dir().borrow();
                    let current_dir = file_system.get_dir(current_dir_idx);
                    current_dir
                        .borrow_mut()
                        .add_file(File::new(parse_result.ok().unwrap()));
                }
            }
            _ => {
                println!("Error on line {}", idx);
            }
        }
    }
    // print_directories_with_files(Rc::new(RefCell::new(file_system)), 0);
    file_system.set_current_dir(0);
    // print_directories_with_files(Rc::new(RefCell::new(file_system)), 0);
    let mut left_dirs: Vec<Rc<RefCell<Directory>>> = Vec::new();
    left_dirs.push(Rc::clone(&file_system.directories.borrow()[0]));
    let mut less_than_100k_sum = 0;
    while !left_dirs.is_empty() {
        let current_dir = left_dirs.pop().unwrap();
        let current_dir_obj = current_dir.borrow();
        let current_dir_size = current_dir_obj.get_size(&file_system);
        if current_dir_size <= 100000 {
            less_than_100k_sum += current_dir_size;
        }
        for sub_dir in current_dir_obj.sub_dirs.borrow().iter() {
            left_dirs
                .push(Rc::clone(&file_system.directories.borrow()[*sub_dir]));
        }
    }

    println!("Solution part 1: {}", less_than_100k_sum);

    return Rc::new(RefCell::new(file_system));
}

fn solve_part_two(contents: &str) {
    let fs = solve_part_one(contents);
    let fs_size = fs.borrow().get_dir(0).borrow().get_size(&fs.borrow());
    let size_diff = (NEEDED_FOR_UPDATE + fs_size - MAX_FS_SIZE) as i32;
    if size_diff >= 0 {
        println!("Enough space for update");
    }

    println!("Solution part 2: {}", size_diff);
    let mut more_than_diff: Vec<u32> = Vec::new();
    let mut left_dirs: Vec<Rc<RefCell<Directory>>> = Vec::new();
    left_dirs.push(Rc::clone(&fs.borrow().directories.borrow()[0]));

    while !left_dirs.is_empty() {
        let current_dir = left_dirs.pop().unwrap();
        let current_dir_obj = current_dir.borrow();
        let current_dir_size = current_dir_obj.get_size(&fs.borrow());
        if current_dir_size as i32 >= size_diff {
            more_than_diff.push(current_dir_size);
        }
        for sub_dir in current_dir_obj.sub_dirs.borrow().iter() {
            left_dirs
                .push(Rc::clone(&fs.borrow().directories.borrow()[*sub_dir]));
        }
    }

    more_than_diff.sort_by(|a, b| a.cmp(b));

    println!("Solution part 2: {}", more_than_diff[0]);
}
