extern crate rand;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use rand::Rng;
use rand::distributions::Alphanumeric;

pub struct TmpFile {
    pub bytes: Vec::<u8>,
    pub path: PathBuf
}

impl TmpFile {
    pub fn new() -> TmpFile {
        let path = TmpFile::rand_path();

        let mut f = File::create(&path).unwrap();
        
        let bytes = rand::thread_rng().gen::<[u8; 32]>();
        f.write_all(&bytes);
        
        let mut vec = Vec::with_capacity(32);
        bytes.iter().for_each(|byte| vec.push(*byte));

        TmpFile { path: PathBuf::from(path), bytes: vec }
    }

    fn rand_path() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .collect::<String>()
    }
}
