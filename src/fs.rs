use std::path::{Path, PathBuf};
use ignore::{Walk, WalkBuilder};
use crate::Config;


pub fn walker<T: AsRef<Path>>(directory: T, config: Option<&Config>) -> Walk {
    let mut builder = WalkBuilder::new(directory);
    builder.add_custom_ignore_filename(&".zipignore");
    if let Some(config) = config {
        builder.max_depth(config.depth)
            .git_ignore(config.read_gitignore)
            .hidden(config.ignore_hidden);
    }
    builder.build()
}

pub fn parent_dir_name<T: AsRef<Path>>(path: T) -> String {
    let absolute = std::fs::canonicalize(path).unwrap();
    let parent_dir = absolute.strip_prefix(absolute.parent().unwrap()).unwrap();
    parent_dir.to_str().unwrap().to_string()
    // parent_dir.to_path_buf()
}


#[derive(Debug)]
pub struct FileCollector {
    base_dir: PathBuf,
    files: Vec<PathBuf>,
    dirs: Vec<PathBuf>,
}

impl FileCollector {
    pub fn new<T: AsRef<Path>>(directory: T) -> FileCollector {
        FileCollector {
            base_dir: PathBuf::from(directory.as_ref()),
            files: vec![],
            dirs: vec![],
        }.prepare(directory)
    }

    fn prepare<T: AsRef<Path>>(mut self, directory: T) -> Self {
        for result in walker(directory, None) {
            match result {
                Ok(s) => {
                    if s.path().is_file() {
                        self.files.push(s.into_path());
                    } else if s.path().is_dir() {
                        self.dirs.push(s.into_path());
                    } else { unreachable!() }
                }
                Err(_) => {}
            }
        }
        self
    }

    pub fn relative_dirs(&self) -> Vec<&Path> {
        let dirs: Vec<&Path> = self.dirs.iter()
            .map(|x| x.as_path().
                strip_prefix(&self.base_dir)
                .unwrap()
            )
            .collect();
        dirs
    }

    pub fn relative_files(&self) -> Vec<&Path> {
        let files: Vec<&Path> = self.files.iter()
            .map(|x| x.as_path().
                strip_prefix(&self.base_dir)
                .unwrap()
            )
            .collect();
        files
    }

    pub fn absolute_files(&self) -> Vec<&Path> {
        let files: Vec<&Path> = self.files.iter()
            .map(|x| x.as_path()
            )
            .collect();
        files
    }

    /// return (absolute, relative)
    pub fn files(&self) -> Vec<(&Path, &Path)> {
        let files: Vec<(&Path, &Path)> = self.files.iter()
            .map(|x| (x.as_path(), x.as_path()
                .strip_prefix(&self.base_dir).unwrap())
            )
            .collect();
        files
    }
}


#[cfg(test)]
mod test {
    use std::fs;
    use std::path::PathBuf;
    use crate::fs::{FileCollector, parent_dir_name};

    #[test]
    fn it_works() {
        let p = parent_dir_name("./");
        println!("{}", p);

        // let collector = FileCollector::new("./");
        // println!("{:?}", collector);
        // let walker = ignore::WalkBuilder::new("./")
        //     .hidden(false)
        //     .build();
        // for x in walker {
        //     println!("{:?}", x);
        // }
    }

    #[test]
    fn relative_dirs_works() {
        let collector = FileCollector::new("./");
        for dirs in collector.relative_dirs() {
            println!("{:?}", dirs);
        }
    }

    #[test]
    fn relative_files_works() {
        let collector = FileCollector::new("./ziptest");
        for files in collector.relative_files() {
            println!("{:?}", files);
        }
    }
}
