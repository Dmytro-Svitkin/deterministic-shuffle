use crate::Collection;
use crate::utils::gcd;

impl<T:Copy,const S:usize>Collection<T,S>{
    pub const unsafe fn multiplicative_shuffle_unchecked(&mut self,multiplier:usize){
        *self=unsafe{self.multiplicative_shuffled_unchecked(multiplier)}
    }

    pub const fn multiplicative_shuffle(&mut self,multiplier:usize){
        *self=self.multiplicative_shuffled(multiplier)
    }

    pub const fn multiplicative_shuffle_soft(&mut self,multiplier:usize){
        *self=self.multiplicative_shuffled_soft(multiplier)
    }

    pub const unsafe fn multiplicative_shuffled_unchecked(&self,multiplier:usize)->Self{
        let initial_collection:[T;S]=self.collection.unwrap();
        let mut result_collection:[T;S]=initial_collection;
        let mut index_counter:usize=0;
        while index_counter<S-1{
            index_counter+=1;
            result_collection[(index_counter*multiplier)%S]=initial_collection[index_counter]
        }
        Self{collection:Some(result_collection)}
    }

    pub const fn multiplicative_shuffled(&self,multiplier:usize)->Self{
        if self.is_none()||gcd(multiplier,S)!=1{return Self{collection:None}}
        unsafe{self.multiplicative_shuffled_unchecked(multiplier)}
    }

    pub const fn multiplicative_shuffled_soft(&self,multiplier:usize)->Self{
        if self.is_none(){return Self{collection:None}}
        else if S<3||gcd(multiplier,S)!=1{return*self}
        unsafe{self.multiplicative_shuffled_unchecked(multiplier)}
    }
}