use std::io::{stdout, Write, BufWriter};

use rand::rngs::{OsRng};
use rand::RngCore;

fn to_stdout(size: usize) {
    let mut rng = OsRng::new().unwrap();

    let bufsize = 8_000_000;
    let mut buf: [u8; bufsize] = [0; bufsize];
    while size > 0 {
        rng.fill_bytes(&mut buf);
        bufwrite.write(&buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
}
