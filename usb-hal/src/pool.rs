//! Allocation pools.

use std::mem::MaybeUninit;


pub struct AllocPool<T, const LEN: usize> {
    pool: [MaybeUninit<T>; LEN],
    links: [u16; LEN],
    head: u16,
}

impl<T, const LEN: usize> AllocPool<T, LEN> {

    pub const fn new() -> Self {
        Self {
            pool: unsafe { MaybeUninit::uninit().assume_init() },
            links: [0; LEN],
            head: 0,
        }
    }

    pub fn alloc(&mut self) -> &mut T
    where 
        T: Default 
    {

        let mut index = self.head;

        todo!()

    }

}
