use crate::Collection;
use crate::utils::reverse_bits;

impl<T:Copy,const S:usize>Collection<T,S>{
    pub const unsafe fn bit_reversal_shuffle_unchecked(&mut self){
        *self=unsafe{self.bit_reversal_shuffled_unchecked()}
    }

    pub const fn bit_reversal_shuffle(&mut self){
        *self=self.bit_reversal_shuffled()
    }

    pub const fn bit_reversal_shuffle_soft(&mut self){
        *self=self.bit_reversal_shuffled_soft()
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
}