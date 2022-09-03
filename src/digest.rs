use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{PathBuf};
use md5::{Md5, Digest};
use sha2::{Sha256};
// use hex_literal::hex;


pub struct ZipDigest {
    path: PathBuf,
    md5: [u8; 16],
    sha256: [u8; 32],
}


impl ZipDigest {
    pub fn new<T: Into<PathBuf>>(path: T) -> Self {
        ZipDigest {
            path: path.into(),
            md5: [0; 16],
            sha256: [0; 32],
        }.md5().sha256()
    }

    pub fn display(&self) {
        println!("{}", self);
    }


    pub fn md5(mut self) -> Self {
        let file = File::open(&self.path).unwrap();
        let mut reader = BufReader::new(file);

        let digest = {
            let mut hasher = Md5::new();
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break; }
                hasher.update(&buffer[..count]);
            }
            hasher.finalize()
        };

        // see https://stackoverflow.com/questions/59376378/how-can-i-turn-a-genericarrayt-into-an-array-of-the-same-length
        self.md5 = digest.as_slice().try_into().unwrap();
        self
    }

    pub fn sha256(mut self) -> Self {
        let file = File::open(&self.path).unwrap();
        let mut reader = BufReader::new(file);

        let digest = {
            let mut hasher = Sha256::new();
            let mut buffer = [0; 1024];
            loop {
                let count = reader.read(&mut buffer).unwrap();
                if count == 0 { break; }
                hasher.update(&buffer[..count]);
            }
            hasher.finalize()
        };

        // see https://stackoverflow.com/questions/59376378/how-can-i-turn-a-genericarrayt-into-an-array-of-the-same-length
        self.sha256 = digest.as_slice().try_into().unwrap();
        self
    }

}


impl Display for ZipDigest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let md5 = base16ct::lower::encode_string(&self.md5);
        write!(f, "MD5   : {}\n", md5)?;
        let sha256 = base16ct::lower::encode_string(&self.sha256);
        write!(f, "SHA256: {}\n", sha256)?;
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use crate::digest::ZipDigest;

    #[test]
    fn it_works() {
        let digest = ZipDigest::new("./ziptest/.zipignore")
            .md5();
        println!("{}", digest);
    }
}