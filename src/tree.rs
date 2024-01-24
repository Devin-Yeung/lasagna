use crate::fs::walker;
use crate::tree_view::{TreeDepth, TreeParams, TreeTrunk};
use crate::Config;
use ignore::Walk;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::vec;

pub fn relative_to(base: &Path, path: &Path) -> Option<usize> {
    for (i, p) in path.ancestors().enumerate() {
        if p == base {
            return Some(i);
        }
    }
    None
}

struct FileWrapper {
    depth: usize,
    is_last: bool,
    path: PathBuf,
}

impl FileWrapper {
    pub fn new(depth: usize, is_last: bool, path: PathBuf) -> FileWrapper {
        FileWrapper {
            depth,
            is_last,
            path,
        }
    }
}

impl std::fmt::Debug for FileWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.depth, self.is_last, self.path.display())
    }
}

pub struct FileStream<'a> {
    items: Vec<Rc<RefCell<FileWrapper>>>,
    config: Option<&'a Config>,
}

impl<'a> FileStream<'a> {
    pub fn new(config: &'a Config) -> FileStream<'a> {
        FileStream::from(walker(&config.input, Some(config)))
    }

    pub fn display(&self) {
        println!("{}", self);
    }
}

impl Display for FileStream<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut tt = TreeTrunk::default();

        for rc in &self.items {
            let params: TreeParams = TreeParams::new(
                TreeDepth(rc.deref().borrow().depth),
                rc.deref().borrow().is_last,
            );
            let tree_part = tt.new_row(params);
            let tree_part: String = tree_part.iter().map(|x| x.ascii_art()).collect();

            let fw = rc.deref().borrow();
            let prefix = fw.path.as_path().parent().unwrap();
            let path = fw.path.strip_prefix(prefix).unwrap();

            writeln!(f, "{} {}", tree_part, path.display())?
        }
        Ok(())
    }
}

impl From<ignore::Walk> for FileStream<'_> {
    fn from(walker: Walk) -> Self {
        let mut items: Vec<Rc<RefCell<FileWrapper>>> = vec![];
        // (Parent, Dir)
        // let mut maybe_last_dir: HashMap<PathBuf, Rc<RefCell<FileWrapper>>> = HashMap::new();
        // let mut maybe_last_file: HashMap<PathBuf, Rc<RefCell<FileWrapper>>> = HashMap::new();
        let mut maybe_last: HashMap<PathBuf, Rc<RefCell<FileWrapper>>> = HashMap::new();

        for entry in walker {
            let entry = entry.unwrap();
            let item = Rc::new(RefCell::new(FileWrapper {
                depth: entry.depth(),
                is_last: false,
                path: entry.into_path(),
            }));

            let wrapper = item.deref().borrow();
            let parent = wrapper.path.parent();
            if parent.is_some() {
                maybe_last.insert(parent.unwrap().to_path_buf(), item.clone());
            }

            // if item.deref().borrow().path.is_dir() {
            //     let wrapper = item.deref().borrow();
            //     let parent = wrapper.path.parent();
            //     if parent.is_some() {
            //         maybe_last_dir.insert(parent.unwrap().to_path_buf(), item.clone());
            //     }
            // }
            //
            // if item.deref().borrow().path.is_file() {
            //     let wrapper = item.deref().borrow();
            //     let parent = wrapper.path.parent();
            //     if parent.is_some() {
            //         maybe_last_file.insert(parent.unwrap().to_path_buf(), item.clone());
            //     }
            // }

            items.push(item.clone());
        }

        // for (_, wrapper) in maybe_last_dir {
        //     wrapper.deref().borrow_mut().is_last = true;
        // }
        // for (_, wrapper) in maybe_last_file {
        //     wrapper.deref().borrow_mut().is_last = true;
        // }

        for (_, wrapper) in maybe_last {
            wrapper.deref().borrow_mut().is_last = true;
        }

        FileStream {
            items,
            config: None,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::tree::relative_to;
    use std::path::Path;

    #[test]
    fn same_dir() {
        let base = Path::new("./");
        let path = Path::new("./");
        assert_eq!(relative_to(base, path).unwrap(), 0);
    }

    #[test]
    fn one_parent() {
        let base = Path::new("./");
        let path = Path::new("./child");
        assert_eq!(relative_to(base, path).unwrap(), 1);
    }

    #[test]
    fn no_parent() {
        let base = Path::new("./parent");
        let path = Path::new("./child/parent");
        assert_eq!(relative_to(base, path), None);
    }

    #[test]
    fn file_test() {
        let base = Path::new("./");
        let path = Path::new("./rustacean.rs");
        assert_eq!(relative_to(base, path).unwrap(), 1);
        let path = Path::new("./rustacean/rustacean.rs");
        assert_eq!(relative_to(base, path).unwrap(), 2);
    }
}
