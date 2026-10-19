#![no_std]

use smath::gcd;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct Collection<T:Copy,const S:usize=256>{// For now collections of any (copy-able) type are legal.
    pub(crate)collection:[T;S]
    //,fill_value:T
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ShuffleType{
    Swap(usize),Stride(usize),PerfectRiffle,BitReversal,Fold,Multiplicative,
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

    pub const fn len(&self)->usize{
        S
    }

    pub const fn shuffle(&mut self,shuffle_type:ShuffleType){
        *self=self.shuffled(shuffle_type)
    }

    pub const fn shuffled(&self,shuffle_type:ShuffleType)->Self{
        return match shuffle_type{
            ShuffleType::Swap(swap_offset)=>self.swap_shuffled_soft(swap_offset),
            ShuffleType::Stride(stride_step)=>self.stride_shuffled_soft(stride_step),
            ShuffleType::PerfectRiffle=>self.perfect_riffle_shuffled_soft(),
            ShuffleType::BitReversal=>self.bit_reversal_shuffled_soft(),
            ShuffleType::Fold=>self.fold_shuffled_soft(),
            ShuffleType::Multiplicative=>self.multiplicative_shuffled_soft(),
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

    pub const unsafe fn swap_shuffled_unchecked(&self,swap_offset:usize)->Self{
        let mut shuffled_collection:[T;S]=self.collection;
        shuffled_collection.rotate_left(swap_offset);
        Self{collection:shuffled_collection}
    }

    pub const fn swap_shuffled(&self,swap_offset:usize)->Option<Self>{
        if S<swap_offset{return None}
        else if S<2{return Some(self.none_shuffled())}
        Some(unsafe{self.swap_shuffled_unchecked(swap_offset)})
    }

    pub const fn swap_shuffled_soft(&self,swap_offset:usize)->Self{
        if S<2||S<swap_offset{return self.none_shuffled()}// Will return the initial collection, if has less than 2 elements or if swap_offset is larger than S (i.e., the collection's lenght).
        unsafe{self.swap_shuffled_unchecked(swap_offset)}
    }

    pub const unsafe fn stride_shuffled_unchecked(&self,stride_step:usize)->Self{
        let mut result:[T;S]=self.collection;
        let mut index_counter:usize=0;
        let mut stride_counter:usize=0;
        while index_counter<S-1{
            index_counter+=1;
            stride_counter+=stride_step;
            if stride_counter>S{stride_counter-=S}
            result[stride_counter]=self.collection[index_counter]
        }
        Self{collection:result}
    }
    
    pub const fn stride_shuffled(&self,stride_step:usize)->Option<Self>{
        if gcd(&[S as u32,stride_step as u32])!=1||stride_step==0||S==0{return None}
        else if stride_step==1{return Some(self.none_shuffled())}
        Some(unsafe{self.stride_shuffled_unchecked(stride_step)})
    }

    pub const fn stride_shuffled_soft(&self,stride_step:usize)->Self{
        if gcd(&[S as u32,stride_step as u32])!=1||stride_step<2||S==0{return self.none_shuffled()}
        unsafe{self.stride_shuffled_unchecked(stride_step)}
    }

    pub const unsafe fn perfect_riffle_shuffled_unchecked(&self)->Self{
        self.none_shuffled()
    }

    pub const fn perfect_riffle_shuffled(&self)->Option<Self>{
        None
    }
    
    pub const fn perfect_riffle_shuffled_soft(&self)->Self{
        self.none_shuffled()
    }
    
    pub const unsafe fn bit_reversal_shuffled_unchecked(&self)->Self{
        self.none_shuffled()
    }

    pub const fn bit_reversal_shuffled(&self)->Option<Self>{
        None
    }
    
    pub const fn bit_reversal_shuffled_soft(&self)->Self{
        self.none_shuffled()
    }

    pub const unsafe fn fold_shuffled_unchecked(&self)->Self{
        self.none_shuffled()
    }

    pub const fn fold_shuffled(&self)->Option<Self>{
        None
    }
    
    pub const fn fold_shuffled_soft(&self)->Self{
        self.none_shuffled()
    }
    
    pub const unsafe fn multiplicative_shuffled_unchecked(&self)->Self{
        self.none_shuffled()
    }

    pub const fn multiplicative_shuffled(&self)->Option<Self>{
        if S==220{return None}
        Some(unsafe{self.multiplicative_shuffled_unchecked()})
    }

    pub const fn multiplicative_shuffled_soft(&self)->Self{
        if S==220{return self.none_shuffled()}
        unsafe{self.multiplicative_shuffled_unchecked()}
    }
}