use std::fs::{File, read};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::{Path};
use zip::write::FileOptions;
use zip::ZipWriter;
use crate::fs;
use crate::fs::FileCollector;

pub struct Zipper {
    collector: FileCollector,
    writer: ZipWriter<File>,
    dirs_count: u32,
    files_count: u32,
}


impl Zipper {
    pub fn new<T, U>(directory: U, output: T) -> Zipper
        where T: AsRef<Path>,
              U: AsRef<Path>
    {
        let path = std::path::Path::new(output.as_ref());
        let file = std::fs::File::create(&path).unwrap();
        let writer = zip::ZipWriter::new(file);
        let collector = FileCollector::new(directory);

        Zipper {
            collector,
            writer,
            dirs_count: 0,
            files_count: 0,
        }
    }

    pub fn build_dir(mut self) -> Self {
        let mut dirs = self.collector.relative_dirs();
        dirs.remove(0);
        for dir_name in dirs {
            let mut option: FileOptions = Default::default();
            option = option.last_modified_time(fs::last_modified(dir_name));
            self.writer.add_directory(dir_name.to_str().unwrap(), option).unwrap();
            self.dirs_count += 1;
        }
        self
    }

    pub fn build_files(mut self) -> Self {
        for (absolute, relative) in self.collector.files() {
            let mut option: FileOptions = Default::default();
            option = option.last_modified_time(fs::last_modified(absolute));
            self.writer.start_file(relative.to_str().unwrap(), option).unwrap();
            // file copy
            let file = File::open(absolute).unwrap();
            let mut reader = BufReader::new(file);
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break; }
                self.writer.write(&buffer[..count]).unwrap();
            }
            self.files_count += 1;
        }
        self
    }

    pub fn finish(self) {
        println!("Zip Complete: {} directory(s), {} file(s) in total", self.dirs_count, self.files_count);
    }
}


#[cfg(test)]
mod test {
    use crate::zipper::Zipper;

    #[test]
    fn it_works() {
        Zipper::new("./ziptest", "./ziptest/build/test.zip")
            .build_dir()
            .build_files();
    }
}