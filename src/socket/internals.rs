use std::io::{Error, ErrorKind, Read, };

use super::Socket;

impl Socket {
    pub(super) fn unshift_bytes(&mut self, bytes: usize) -> [u8; Socket::BUFSIZE] {
        let mut buf: [u8; Socket::BUFSIZE] = [0; Socket::BUFSIZE];
        match self.filelock.file.read(&mut buf) {
            Err(e) => panic!("Couldn't read from keyfile: {}", e),
            Ok(read_bytes) => {
                if read_bytes != bytes { panic!("Couldn't read enough bytes") }
            }
        };
        buf
    }

    pub(super) fn check_len(&self, len: usize) -> Result<(), Error> {
        if len <= Socket::BUFSIZE {
            return Ok(());
        } else {
            Err(Error::new(
                ErrorKind::Other, format!("can only send up to {}B", Socket::BUFSIZE)
            ))
        }
    }

    pub(super) fn clear_key(&self, bytes: usize) -> usize {
        self.filelock.file.seek(SeekFrom::Start(bytes));
    }
}
