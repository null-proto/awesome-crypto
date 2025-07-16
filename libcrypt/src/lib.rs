#![allow(unused)]
#![allow(dead_code)]

use std::{io::{stdin, Read, Seek, SeekFrom, Write}, path::PathBuf};


pub struct Cryptic {
    path : PathBuf,
}

impl Cryptic {
    pub fn new(path : &str) -> Self {
        Self {
            path : PathBuf::from(path),
        }
    }

    pub fn encrypt(&self, enc_fn : impl CryptFunction) {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(self.path.clone())
            .expect("cannot open");
        let mut key_path = self.path.clone();
        key_path.set_extension("key");
        let mut key = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open( key_path)
            .expect("cannot open");

        key.write_all(enc_fn.get_signature());

        let mut buf = [0u8;4096];
        let mut seek = 0usize;
        'enc: loop {
            match file.read(&mut buf) {
                Err(e) => panic!("PANIC!!! {}",e),
                Ok(0) => break 'enc,
                Ok(len) => {
                    let cp = file.stream_position().expect("seek error");
                    enc_fn.encrypt(&mut buf[..len]);
                    file.seek(SeekFrom::Start(cp - ((len) as u64)));
                    file.write(&buf[..len]).expect("write error");
                    if (len < 4096) {
                        break 'enc;
                    } else {
                        file.seek(SeekFrom::Start(cp));
                    }
                }
            }
        }
    }


    pub fn decrypt(&self, enc_fn : &mut impl CryptFunction) {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .read(true)
            .open(self.path.clone())
            .expect("cannot open source file");
        let mut key_path = self.path.clone();
        key_path.set_extension("key");
        let mut key = std::fs::OpenOptions::new()
            .create(false)
            .read(true)
            .open(key_path)
            .expect("cannot open key");

        let mut sig : Vec<u8> = Vec::new();
        key.read_to_end(&mut sig);
        enc_fn.set_signature(&sig);

        let mut buf = [0u8;4096];
        let mut seek = 0usize;
        'dec: loop {
            match file.read(&mut buf) {
                Ok(0) => break 'dec,
                Err(e) => panic!("PANIC!!! {}",e),
                Ok(len) => {
                    let cp = file.stream_position().expect("seek error");
                    enc_fn.decrypt(&mut buf[..len]);
                    file.seek(SeekFrom::Start(cp - ((len) as u64)));
                    file.write(&buf[..len]).expect("write error");
                    file.seek(SeekFrom::Start(cp));
                    if (len < 4096) {
                        break 'dec;
                    } else {
                        file.seek(SeekFrom::Start(cp));
                    }
                }
            }
        }
    }

}


pub trait CryptFunction {
    fn encrypt(&self,data:&mut [u8]);
    fn decrypt(&self,data:&mut [u8]);
    fn get_signature(&self) -> &[u8];
    fn set_signature(&mut self, sig: &[u8]);
}
