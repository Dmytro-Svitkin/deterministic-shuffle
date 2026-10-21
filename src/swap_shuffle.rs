use crate::Collection;

impl<T:Copy,const S:usize>Collection<T,S>{
    pub const unsafe fn swap_shuffle_unchecked(&mut self,swap_offset:usize){
        *self=unsafe{self.swap_shuffled_unchecked(swap_offset)}
    }

    pub const fn swap_shuffle(&mut self,swap_offset:usize){
        *self=self.swap_shuffled(swap_offset)
    }

    pub const fn swap_shuffle_soft(&mut self,swap_offset:usize){
        *self=self.swap_shuffled_soft(swap_offset)
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
        else if S<2||S<swap_offset{return*self}// Returns the initial collection, if has less than 2 elements or if swap_offset is larger than S (i.e., the collection's lenght).
        unsafe{self.swap_shuffled_unchecked(swap_offset)}
    }
}