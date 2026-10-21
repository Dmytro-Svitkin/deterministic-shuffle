use crate::Collection;

impl<T:Copy,const S:usize>Collection<T,S>{
    pub const unsafe fn fold_shuffle_unchecked(&mut self){
        *self=unsafe{self.fold_shuffled_unchecked()}
    }

    pub const fn fold_shuffle(&mut self){
        *self=self.fold_shuffled()
    }

    pub const fn fold_shuffle_soft(&mut self){
        *self=self.fold_shuffled_soft()
    }

    pub const unsafe fn fold_shuffled_unchecked(&self)->Self{
        let initial_collection:[T;S]=self.collection.unwrap();
        let mut result_collection:[T;S]=initial_collection;
        let half:usize=S/2;
        let mut half_counter:usize=0;
        while half_counter<half{
            result_collection[half_counter*2]=initial_collection[half_counter];
            result_collection[half_counter*2+1]=initial_collection[S-half_counter-1];
            half_counter+=1;
        }
        Self{collection:Some(result_collection)}
    }

    pub const fn fold_shuffled(&self)->Self{
        if self.is_none()||S%2!=0{return Self{collection:None}}
        else if S==2{return*self}
        unsafe{self.fold_shuffled_unchecked()}
    }

    pub const fn fold_shuffled_soft(&self)->Self{
        if self.is_none(){return Self{collection:None}}
        else if S<3{return*self}
        else if S%2!=0{
            let mut result_collection:[T;S]=unsafe{self.fold_shuffled_unchecked()}.collection.unwrap();
            result_collection[S-1]=self.collection.unwrap()[S/2];
            return Self{collection:Some(result_collection)}
        }
        unsafe{self.fold_shuffled_unchecked()}
    }
}