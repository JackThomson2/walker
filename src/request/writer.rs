use std::io;

use bytes::BufMut;

pub struct Writer<'a, B>(pub &'a mut B);

impl<'a, B: BufMut> io::Write for Writer<'a, B> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.put_slice(buf);
        Ok(buf.len())
    }
    #[inline]
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
