#![no_std]

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
struct Collection<T,const S:usize=256>{// For now collections of any type are legal.
    pub(crate)collection:[T;S]
    //,fill_value:T
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum ShuffleType{
    Stride,PerfectRiffle,BitReversal,Fold,Multiplicative,
    None// Also a shuffle, which does not permutate the initial collection.
}

impl<T:Copy,const S:usize>Collection<T,S>{// T needs to be copy-able.
    pub const fn from(collection:[T;S])->Self{
        Self{collection}
    }

    pub const fn as_array(&self)->&[T]{
        &self.collection
    }

    pub fn to_raw_array(self)->[T;S]{
        self.collection
    }

    pub const fn shuffle(&mut self,shuffle_type:ShuffleType){
    }

    pub const fn shuffled(&self,shuffle_type:ShuffleType)->Self{
        Self{collection:self.collection}
    }

    pub const fn auto_shuffle(&mut self){// Uses the best* shuffle for the current collection.
    }

    pub const fn auto_shuffled(&self)->Self{// Uses the best* shuffle for the current collection.
        Self{collection:self.collection}
    }
}