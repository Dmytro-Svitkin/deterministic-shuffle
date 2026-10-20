#![no_std]

use smath::gcd;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct Collection<T:Copy,const S:usize=256>{// For now collections of any (copy-able) type are legal.
    pub(crate)collection:Option<[T;S]>
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ShuffleType{
    Swap(usize),Stride(usize),PerfectRiffle,BitReversal,Fold,Multiplicative,
    None// Also a shuffle, which does not permutate the initial collection.
}

impl<T:Copy,const S:usize>Collection<T,S>{// T needs to be copy-able.
    pub const fn from(collection:[T;S])->Self{
        Self{collection:Some(collection)}
    }

    pub const fn from_raw(raw_collection:Option<[T;S]>)->Self{
        Self{collection:raw_collection}
    }

    pub const fn as_raw(&self)->&Option<[T;S]>{
        &self.collection
    }

    pub const fn to_raw(&self)->Option<[T;S]>{
        self.collection
    }

    pub const unsafe fn to_array_unchecked(&self)->[T;S]{
        self.collection.unwrap()
    }

    pub const fn to_array(&self)->Option<[T;S]>{
        self.to_raw()
    }

    pub const fn is_none(&self)->bool{
        self.collection.is_none()
    }

    pub const unsafe fn len_unchecked(&self)->usize{
        S
    }

    pub const fn len(&self)->Option<usize>{
        if self.is_none(){return None}
        Some(S)
    }

    pub const fn len_soft(&self)->usize{
        if self.is_none(){return 0}
        S
    }

    pub const fn shuffle(&mut self,shuffle_type:ShuffleType){
        *self=self.shuffled(shuffle_type)
    }

    pub const fn shuffle_soft(&mut self,shuffle_type:ShuffleType){
        *self=self.shuffled_soft(shuffle_type)
    }

    pub const fn shuffled(&self,shuffle_type:ShuffleType)->Self{
        return match shuffle_type{
            ShuffleType::Swap(swap_offset)=>self.swap_shuffled(swap_offset),
            ShuffleType::Stride(stride_step)=>self.stride_shuffled(stride_step),
            ShuffleType::PerfectRiffle=>self.perfect_riffle_shuffled(),
            ShuffleType::BitReversal=>self.bit_reversal_shuffled(),
            ShuffleType::Fold=>self.fold_shuffled(),
            ShuffleType::Multiplicative=>self.multiplicative_shuffled(),
            _=>*self
        }
    }
    
    pub const fn shuffled_soft(&self,shuffle_type:ShuffleType)->Self{
        return match shuffle_type{
            ShuffleType::Swap(swap_offset)=>self.swap_shuffled_soft(swap_offset),
            ShuffleType::Stride(stride_step)=>self.stride_shuffled_soft(stride_step),
            ShuffleType::PerfectRiffle=>self.perfect_riffle_shuffled_soft(),
            ShuffleType::BitReversal=>self.bit_reversal_shuffled_soft(),
            ShuffleType::Fold=>self.fold_shuffled_soft(),
            ShuffleType::Multiplicative=>self.multiplicative_shuffled_soft(),
            _=>*self
        }
    }

    pub const fn auto_shuffle(&mut self){// Uses the best* shuffle for the current collection.
    }

    pub const fn auto_shuffled(&self)->Self{// Uses the best* shuffle for the current collection.
        Self{collection:self.collection}
    }

    pub const unsafe fn swap_shuffled_unchecked(&self,swap_offset:usize)->Self{
        let mut result_collection:[T;S]=self.collection.unwrap();
        result_collection.rotate_left(swap_offset);
        Self{collection:Some(result_collection)}
    }

    pub const fn swap_shuffled(&self,swap_offset:usize)->Self{
        if self.is_none()||S<swap_offset{return Self{collection:None}}
        else if S<2{return*self}
        unsafe{self.swap_shuffled_unchecked(swap_offset)}
    }

    pub const fn swap_shuffled_soft(&self,swap_offset:usize)->Self{
        if self.is_none(){return Self{collection:None}}
        else if S<2||S<swap_offset{return*self}// Will return the initial collection, if has less than 2 elements or if swap_offset is larger than S (i.e., the collection's lenght).
        unsafe{self.swap_shuffled_unchecked(swap_offset)}
    }

    pub const unsafe fn stride_shuffled_unchecked_overflow_vulnerable(&self,stride_step:usize)->Self{
        let initial_collection:[T;S]=self.collection.unwrap();
        let mut result_collection:[T;S]=initial_collection;
        let mut index_counter:usize=0;
        let mut stride_counter:usize=0;
        while index_counter<S-1{
            index_counter+=1;
            stride_counter+=stride_step;
            if stride_counter>S{stride_counter-=S}
            result_collection[stride_counter]=initial_collection[index_counter]
        }
        Self{collection:Some(result_collection)}
    }

    pub const unsafe fn stride_shuffled_unchecked(&self,stride_step:usize)->Self{
        let initial_collection:[T;S]=self.collection.unwrap();
        let mut result_collection:[T;S]=initial_collection;
        let mut index_counter:usize=0;
        let mut stride_counter:usize=0;
        while index_counter<S-1{
            index_counter+=1;
            if S-stride_counter<=stride_step{stride_counter=stride_step-(S-stride_counter)}// Simple overflow protection.
            else{stride_counter+=stride_step}
            result_collection[stride_counter]=initial_collection[index_counter]
        }
        Self{collection:Some(result_collection)}
    }
    
    pub const fn stride_shuffled(&self,stride_step:usize)->Self{
        if self.is_none()||gcd(&[S as u32,stride_step as u32])!=1||stride_step==0||stride_step>=S{return Self{collection:None}}// `S==0` and `stride_step>=S` conditions unified into `stride_step>=S`.
        else if stride_step==1{return *self}
        unsafe{self.stride_shuffled_unchecked(stride_step)}
    }

    pub const fn stride_shuffled_soft(&self,stride_step:usize)->Self{
        if self.is_none(){return Self{collection:None}}
        else if gcd(&[S as u32,stride_step as u32])!=1||stride_step<2||stride_step>=S{return *self}// `S==0` and `stride_step>=S` conditions unified into `stride_step>=S`.
        unsafe{self.stride_shuffled_unchecked(stride_step)}
    }

    pub const unsafe fn perfect_riffle_shuffled_unchecked(&self)->Self{
        let initial_collection:[T;S]=self.collection.unwrap();
        let mut result:[T;S]=initial_collection;
        let half:usize=S/2;
        let mut half_counter:usize=0;
        while half_counter<half{
            result[half_counter*2]=initial_collection[half_counter];
            result[half_counter*2+1]=initial_collection[half_counter+half];
            half_counter+=1;
        }
        Self{collection:Some(result)}
    }

    pub const fn perfect_riffle_shuffled(&self)->Self{
        if self.is_none()||S%2!=0{return Self{collection:None}}
        else if S<3{return*self}
        unsafe{self.perfect_riffle_shuffled_unchecked()}
    }

    pub const fn perfect_riffle_shuffled_soft(&self)->Self{
        if self.is_none(){return Self{collection:None}}
        else if S<3{return*self}
        else if S%2!=0{return unsafe{self.stride_shuffled_unchecked(2)}}
        unsafe{self.perfect_riffle_shuffled_unchecked()}
    }

    pub const unsafe fn bit_reversal_shuffled_unchecked(&self)->Self{
        let initial_collection:[T;S]=self.collection.unwrap();
        let mut result_collection:[T;S]=initial_collection;
        let anchor_value_bits:u32=S.trailing_zeros();
        let mut index_counter:usize=0;
        while index_counter<S-2{
            index_counter+=1;
            result_collection[reverse_bits(index_counter,anchor_value_bits)]=initial_collection[index_counter]
        }
        Self{collection:Some(result_collection)}
    }

    pub const fn bit_reversal_shuffled(&self)->Self{
        if self.is_none()||!S.is_power_of_two(){return Self{collection:None}}
        else if S==1{return*self}
        unsafe{self.bit_reversal_shuffled_unchecked()}
    }

    pub const fn bit_reversal_shuffled_soft(&self)->Self{
        if self.is_none(){return Self{collection:None}}
        else if S==1||!S.is_power_of_two(){return*self}
        unsafe{self.bit_reversal_shuffled_unchecked()}
    }

    pub const unsafe fn fold_shuffled_unchecked(&self)->Self{
        *self
    }

    pub const fn fold_shuffled(&self)->Self{
        *self
    }

    pub const fn fold_shuffled_soft(&self)->Self{
        *self
    }

    pub const unsafe fn multiplicative_shuffled_unchecked(&self)->Self{
        *self
    }

    pub const fn multiplicative_shuffled(&self)->Self{
        *self
    }

    pub const fn multiplicative_shuffled_soft(&self)->Self{
        *self
    }
}

const fn reverse_bits(mut value:usize,anchor_value_bits:u32)->usize{
    let mut result:usize=0;
    let mut counter:u32=0;
    while counter<anchor_value_bits{
        result=(result<<1)|(value&1);
        value>>=1;
        counter+=1
    }
    result
}