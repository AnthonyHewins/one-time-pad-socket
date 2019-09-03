use std::io::{stdout, Write, BufWriter};

use rand::rngs::{OsRng};
use rand::RngCore;

pub fn to_stdout(kb_size: usize) -> std::io::Result<()> {
    let mut rng = OsRng::new().unwrap();

    if (kb_size > 100) {
        panic!("Too big, pick something smaller than 100");
    }

    let mut buf: [u8; 1024] = [0; 1024];
    let mut out = stdout();
    for _ in 0..kb_size {
        rng.fill_bytes(&mut buf);
        out.write(&buf)?;
    }

    Ok(())
}
