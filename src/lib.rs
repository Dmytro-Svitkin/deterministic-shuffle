#![no_std]

struct Collection<T,const S:usize=256>{// For now collections of any type are legal.
    pub(crate)collection:[T;S]
}

impl<T,const S:usize>Collection<T,S>{
}