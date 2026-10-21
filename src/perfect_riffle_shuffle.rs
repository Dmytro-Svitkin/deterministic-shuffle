use crate::Collection;

impl<T:Copy,const S:usize>Collection<T,S>{
    pub const unsafe fn perfect_riffle_shuffle_unchecked(&mut self){
        *self=unsafe{self.perfect_riffle_shuffled_unchecked()}
    }

    pub const fn perfect_riffle_shuffle(&mut self){
        *self=self.perfect_riffle_shuffled()
    }

    pub const fn perfect_riffle_shuffle_soft(&mut self){
        *self=self.perfect_riffle_shuffled_soft()
    }

    pub const unsafe fn perfect_riffle_shuffled_unchecked(&self)->Self{
        let initial_collection:[T;S]=self.collection.unwrap();
        let mut result_collection:[T;S]=initial_collection;
        let half:usize=S/2;
        let mut half_counter:usize=0;
        while half_counter<half{
            result_collection[half_counter*2]=initial_collection[half_counter];
            result_collection[half_counter*2+1]=initial_collection[half_counter+half];
            half_counter+=1;
        }
        Self{collection:Some(result_collection)}
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
}