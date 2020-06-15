use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    data: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            data: wrapped,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        match self.data.read(buf) {
            Ok(x) => {
                self.bytes += x;
                Ok(x)
            }
            Err(why) => panic!("{:?}", why),
        }
    }
}

pub struct WriteStats<W> {
    data: W,
    writes: usize,
    bytes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            data: wrapped,
            writes: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.data
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        match self.data.write(buf) {
            Ok(x) => {
                self.bytes += x;
                Ok(x)
            }
            Err(why) => panic!("{:?}", why),
        }
    }

    fn flush(&mut self) -> Result<()> {
        self.data.flush()
    }
}
