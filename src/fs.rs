use std::fs;
use std::path::{Path, PathBuf};
use ignore::{Walk, WalkBuilder};
use time::macros::offset;
use time::{OffsetDateTime, UtcOffset};
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

pub fn last_modified<T: AsRef<Path>>(path: T) -> zip::DateTime {
    let meta = fs::metadata(path).unwrap();
    let system_time = meta.modified().unwrap();
    let offset_time = OffsetDateTime::from(system_time);
    let local_offset = UtcOffset::current_local_offset().unwrap();
    let local_time =  offset_time.to_offset(local_offset);
    zip::DateTime::from_time(local_time).unwrap()
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

    /// return (absolute, relative)
    pub fn dirs(&self) -> Vec<(&Path, &Path)> {
        let dirs: Vec<(&Path, &Path)> = self.dirs.iter()
            .map(|x| (x.as_path(), x.as_path()
                .strip_prefix(&self.base_dir).unwrap())
            )
            .collect();
        dirs
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
