// Just trying to implement the best idea in community solutions

use std::io::{Read, Result, Write};

pub type ReadStats<T> = IoStats<T>;
pub type WriteStats<T> = IoStats<T>;

pub struct IoStats<T> {
    wrapped: T,
    operations: usize,
    bytes: usize,
}

impl<T> IoStats<T> {
    pub fn new(_wrapped: T) -> IoStats<T> {
        Self {
            wrapped: _wrapped,
            operations: 0,
            bytes: 0,
        }
    }

    pub fn get_ref(&self) -> &T {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    // if only we had method alias syntax...
    pub fn reads(&self) -> usize {
        self.operations
    }

    pub fn writes(&self) -> usize {
        self.operations
    }
}

impl<R: Read> Read for IoStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.operations += 1;
        let read_bytes = self.wrapped.read(buf)?;
        self.bytes += read_bytes;
        Ok(read_bytes)
    }
}

impl<W: Write> Write for IoStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.operations += 1;
        let written_bytes = self.wrapped.write(buf)?;
        self.bytes += written_bytes;
        Ok(written_bytes)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
