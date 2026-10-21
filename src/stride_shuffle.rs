use crate::Collection;
use crate::utils::gcd;

impl<T:Copy,const S:usize>Collection<T,S>{
    pub const unsafe fn stride_shuffle_unchecked(&mut self,stride_step:usize){
        *self=unsafe{self.stride_shuffled_unchecked(stride_step)}
    }

    pub const fn stride_shuffle(&mut self,stride_step:usize){
        *self=self.stride_shuffled(stride_step)
    }

    pub const fn stride_shuffle_soft(&mut self,stride_step:usize){
        *self=self.stride_shuffled_soft(stride_step)
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
        if self.is_none()||gcd(S,stride_step)!=1||stride_step==0||stride_step>=S{return Self{collection:None}}// `S==0` and `stride_step>=S` conditions unified into `stride_step>=S`.
        else if stride_step==1{return *self}
        unsafe{self.stride_shuffled_unchecked(stride_step)}
    }

    pub const fn stride_shuffled_soft(&self,stride_step:usize)->Self{
        if self.is_none(){return Self{collection:None}}
        else if gcd(S,stride_step)!=1||stride_step<2||stride_step>=S{return *self}// `S==0` and `stride_step>=S` conditions unified into `stride_step>=S`.
        unsafe{self.stride_shuffled_unchecked(stride_step)}
    }
}