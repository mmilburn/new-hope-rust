use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};

fn randombytes(xlen: usize) -> Vec<u8>
{
    let mut result = read_bytes(xlen);
    while result.is_err()
    {
        thread::sleep(time::Duration::from_secs(1));
        result = read_bytes(xlen);
    }

    return result.unwrap();
}

fn read_bytes(xlen: usize) -> Result<Vec<u8>, io::Error>
{
    let input = File::open("/dev/urandom")?;
    let mut buf: Vec<u8> = Vec::new();
    let mut handle = input.take(xlen as u64);
    handle.read_to_end(&mut buf)?;
    return Ok(buf);
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_got_bytes()
    {
        let wanted_size = 128;
        let bytes = randombytes(wanted_size);
        assert_eq!(bytes.len() == wanted_size, true, 
                   "requested {} bytes got {}", wanted_size, bytes.len());
    }
}
