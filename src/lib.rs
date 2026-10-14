#![no_std]

struct Collection<T,const S:usize=256>{// For now collections of any type are legal.
    pub(crate)collection:[T;S]
    //,default_value:T
}

impl<T,const S:usize>Collection<T,S>{
    pub const fn from(collection:[T;S])->Self{
        Self{collection}
    }

    pub const fn as_array(&self)->&[T]{
        &self.collection
    }

    pub fn to_raw_array(self)->[T;S]{
        self.collection
    }
}