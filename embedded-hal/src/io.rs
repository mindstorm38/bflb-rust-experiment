//! No-std IO traits.


pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, ()>;
}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, ()>;
}
