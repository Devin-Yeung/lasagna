use crate::fs::FileCollector;
use crate::{fs, Config};
use std::fs::File;
use std::io::{BufReader, Read, Write};

use zip::write::FileOptions;
use zip::ZipWriter;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

pub struct Zipper<'a> {
    config: &'a Config,
    collector: FileCollector,
    writer: ZipWriter<File>,
    dirs_count: u32,
    files_count: u32,
}

impl<'a> Zipper<'a> {
    pub fn new(config: &'a Config) -> Zipper {
        let path = std::path::Path::new(&config.output);
        let file = std::fs::File::create(&path).unwrap();
        let writer = zip::ZipWriter::new(file);
        let collector = FileCollector::new(&config.input, Some(config));

        Zipper {
            collector,
            writer,
            dirs_count: 0,
            files_count: 0,
            config,
        }
    }

    pub fn build_dir(mut self) -> Self {
        let mut dirs = self.collector.dirs();
        dirs.remove(0);
        for (absolute, relative) in dirs {
            let mut option: FileOptions =
                FileOptions::default().last_modified_time(fs::last_modified(absolute));

            #[cfg(unix)]
            {
                let permission = std::fs::File::open(absolute)
                    .unwrap()
                    .metadata()
                    .unwrap()
                    .permissions()
                    .mode();
                option = option.unix_permissions(permission);
            }

            let name = if self.config.parent {
                format!(
                    "{}/{}",
                    self.config.output.file_stem().unwrap().to_str().unwrap(),
                    relative.to_str().unwrap()
                )
            } else {
                format!("{}", relative.to_str().unwrap())
            };
            self.writer.add_directory(name, option).unwrap();
            self.dirs_count += 1;
        }
        self
    }

    pub fn build_files(mut self) -> Self {
        for (absolute, relative) in self.collector.files() {
            let mut option: FileOptions =
                FileOptions::default().last_modified_time(fs::last_modified(absolute));

            #[cfg(unix)]
            {
                let permission = std::fs::File::open(absolute)
                    .unwrap()
                    .metadata()
                    .unwrap()
                    .permissions()
                    .mode();
                option = option.unix_permissions(permission);
            }

            let name = if self.config.parent {
                format!(
                    "{}/{}",
                    self.config.output.file_stem().unwrap().to_str().unwrap(),
                    relative.to_str().unwrap()
                )
            } else {
                format!("{}", relative.to_str().unwrap())
            };
            self.writer.start_file(name, option).unwrap();
            // file copy
            let file = File::open(absolute).unwrap();
            let mut reader = BufReader::new(file);
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 {
                    break;
                }
                self.writer.write_all(&buffer[..count]).unwrap();
            }
            self.files_count += 1;
        }
        self
    }

    pub fn finish(self) {
        println!(
            "Zip Complete: {} directory(s), {} file(s) in total",
            self.dirs_count, self.files_count
        );
    }
}
