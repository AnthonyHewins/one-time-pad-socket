use std::io::{
    BufWriter,
    Error,
    ErrorKind,
    Write,
};

use super::Socket;

// TODO: make it handle more than 8KiB
// TODO: draining the key needs more safeguards

impl Socket {
    pub(super) fn xor(&self, contents: &[u8], encrypted: &mut [u8]) -> Result<(), Error>{
        let len = contents.len();
        self.check_len(len)?;

        for i in 0..len {
            encrypted[i] = self.key[i] ^ contents[i];
        }

        Ok(())
    }

    pub(super) fn check_len(&self, len: usize) -> Result<(), Error> {
        if len > Socket::BUFSIZE {
            return Err(Error::new(
                ErrorKind::Other,
                format!("can only send up to {}B. Will be fixed in the future.", Socket::BUFSIZE)
            ));
        }

        let keysize = self.key.len();
        if len <= keysize {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("bufsize is {} but key is only {}", len, keysize)
            ));
        }

        Ok(())
    }

    pub(super) fn drain_key(&mut self, len: usize) {
        self.key.drain(0..len);
        let mut stream = BufWriter::new(&self.filelock.file);
        self.key.iter().map(|i| stream.write_all(&[*i]));
    }

    //pub(super) fn clear_key(&self, bytes: usize) -> usize {
    //    let mut tmp = [0; Socket::BUFSIZE];
    //    self.filelock.file.seek(SeekFrom::Start(bytes.try_into().unwrap()));

    //    let bytes_read = self.filelock.file.read(&mut tmp);
    //    self.filelock.file.write_all
    //}
}
