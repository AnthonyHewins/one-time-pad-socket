use std::fs::File;
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
        if len > keysize {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!("bufsize is {} but key is only {}", len, keysize)
            ));
        }

        Ok(())
    }

    pub(super) fn drain_key(&mut self, len: usize) {
        self.key.drain(0..len);

        let mut truncated_file = BufWriter::new(File::create(&self.file).unwrap());
        self.key.iter().for_each(|i| truncated_file.write_all(&[*i]).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Read;

    fn new_socket() -> Socket {
        let tmp = util::fs::TmpFile::new();
        let cs = util::net::ClientServer::new();
        Socket::new(cs.server, tmp.path).unwrap()
    }

    #[test]
    fn test_xor_happy_path() {
        let socket = new_socket();

        let mut plaintext: [u8; 8] = [0; 8];
        let mut cipher: [u8; 8] = [0; 8];
        socket.xor(&plaintext, &mut cipher).unwrap();

        // Encrypt plaintext to test if it worked
        for i in 0..plaintext.len() {
            plaintext[i] = plaintext[i] ^ socket.key[i];
        }

        assert_eq!(plaintext, cipher);
    }

    #[test]
    fn test_xor_key_too_short() {
        let socket = new_socket();

        let plaintext: [u8; 800] = [0; 800];
        let mut cipher: [u8; 800] = [0; 800];

        assert!(
            match socket.xor(&plaintext, &mut cipher) {
                Err(e) => e.kind() == ErrorKind::UnexpectedEof,
                _ => false
            }
        );
    }
    
    #[test]
    fn drain_key() {
        let mut socket = new_socket();

        let original = socket.key.len();
        socket.drain_key(3);

        assert_eq!(socket.key.len(), original - 3);
    }

    #[test]
    fn it_drains_keyfile_bytes() {
        let mut socket = new_socket();

        let original = socket.key.len();
        socket.drain_key(3);

        let mut key = File::open(&socket.file).unwrap();
        let mut buf = [0; 500];
        assert_eq!(key.read(&mut buf).unwrap(), original - 3);
    }
}
