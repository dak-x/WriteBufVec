use std::io::Write;

/// Wraps a writer and buffers its output.
///
/// The size of the <a href = "https://doc.rust-lang.org/std/io/struct.BufWriter.html"> `std::io::BufWriter` </a> is not large enough for the <a href = "http://github.com/dak-x/ray-tracing" > ray_tracing </a>. This crate provides a larger buffer with the max capacity of `1MB`. The usage is exactly similar to <a href = "https://doc.rust-lang.org/std/io/struct.BufWriter.html"> `BufWriter` </a> and it functions like a replacement for the same.
///
/// Example:
/// ```
/// use write_buf::*;
/// use std::io::{stdout,Write};
/// let mut writer = WriteBufVec::new(stdout());
///
/// for i in 0..10 {
/// writer.write(&[i+1]).unwrap();
/// }
/// assert!(writer.flush().is_ok());
/// ```

pub struct WriteBufVec<T: Write> {
    len: usize,
    buf: Vec<u8>,
    writer: T,
}

impl<T: Write> WriteBufVec<T> {
    /// Outputs a new `writer` wrapping around the input `writer`
    pub fn new(writer: T) -> Self {
        WriteBufVec {
            len: 0,
            buf: Vec::new(),
            writer,
        }
    }
    /// The amount of written bytes currently inside the buffer.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T: Write> Write for WriteBufVec<T> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let l = buf.len();
        if self.len + l < 1024 * 1024 {
            self.len += l;
            self.buf.extend_from_slice(buf);
            Ok(l)
        } else {
            self.flush()?;
            self.buf.clear();
            self.buf.extend_from_slice(buf);
            self.len = l;
            Ok(l)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.writer.write(self.buf.as_slice())?;
        Ok(())
    }
}
