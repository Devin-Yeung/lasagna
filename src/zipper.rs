use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::{Path};
use zip::ZipWriter;
use crate::fs::FileCollector;

pub struct Zipper {
    collector: FileCollector,
    writer: ZipWriter<File>,
    dirs_count: u32,
    files_count:u32,
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
            files_count: 0
        }
    }

    pub fn build_dir(mut self) -> Self {
        for dir_name in self.collector.relative_dirs() {
            self.writer.add_directory(dir_name.to_str().unwrap(), Default::default()).expect("TODO");
            self.dirs_count += 1;
        }
        self
    }

    pub fn build_files(mut self) -> Self {
        for (absolute, relative) in self.collector.files() {
            self.writer.start_file(relative.to_str().unwrap(), Default::default()).expect("TODO");
            // file copy
            let file = File::open(absolute).expect("TODO");
            let mut reader = BufReader::new(file);
            let buf = reader.fill_buf().unwrap();
            self.writer.write_all(buf).expect("TODO");
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