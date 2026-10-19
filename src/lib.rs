#![no_std]

use smath::gcd;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct Collection<T:Copy+PartialEq,const S:usize=256>{// For now collections of any (copy-able) type are legal.
    pub(crate)collection:[T;S]
    //,fill_value:T
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ShuffleType{
    Swap(usize),Stride(usize),PerfectRiffle,BitReversal,Fold,Multiplicative,
    None// Also a shuffle, which does not permutate the initial collection.
}

impl<T:Copy+PartialEq,const S:usize>Collection<T,S>{// T needs to be copy-able.
    pub const fn from(collection:[T;S])->Self{
        Self{collection}
    }

    pub const fn as_array(&self)->&[T]{
        &self.collection
    }

    pub fn to_raw_array(self)->[T;S]{
        self.collection
    }

    pub const fn len(&self)->usize{
        S
    }

    pub const fn shuffle(&mut self,shuffle_type:ShuffleType){
        *self=self.shuffled(shuffle_type)
    }

    pub const fn shuffled(&self,shuffle_type:ShuffleType)->Self{
        return match shuffle_type{
            ShuffleType::Swap(swap_offset)=>self.swap_shuffled(swap_offset),
            ShuffleType::Stride(stride_step)=>self.stride_shuffled(stride_step),
            ShuffleType::PerfectRiffle=>self.perfect_riffle_shuffled(),
            ShuffleType::BitReversal=>self.bit_reversal_shuffled(),
            ShuffleType::Fold=>self.fold_shuffled(),
            ShuffleType::Multiplicative=>self.multiplicative_shuffled(),
            ShuffleType::None=>self.none_shuffled()
        }
    }

    pub const fn auto_shuffle(&mut self){// Uses the best* shuffle for the current collection.
    }

    pub const fn auto_shuffled(&self)->Self{// Uses the best* shuffle for the current collection.
        Self{collection:self.collection}
    }

    const fn none_shuffled(&self)->Self{
        *self
    }

    const fn swap_shuffled(&self,swap_offset:usize)->Self{
        if S<2||S<swap_offset{return self.none_shuffled()}// Will return the initial collection, if has less than 2 elements or if swap_offset is larger than S (i.e., the collection's lenght).
        let mut shuffled_collection:[T;S]=self.collection;
        shuffled_collection.rotate_left(swap_offset);
        Self{collection:shuffled_collection}
    }

    const fn stride_shuffled(&self,stride_step:usize)->Self{
        if gcd(&[S as u32,stride_step as u32])!=1{return self.none_shuffled()}
        self.none_shuffled()
    }

    const fn perfect_riffle_shuffled(&self)->Self{
        self.none_shuffled()
    }

    const fn bit_reversal_shuffled(&self)->Self{
        self.none_shuffled()
    }

    const fn fold_shuffled(&self)->Self{
        self.none_shuffled()
    }
    
    const fn multiplicative_shuffled(&self)->Self{
        self.none_shuffled()
    }
}