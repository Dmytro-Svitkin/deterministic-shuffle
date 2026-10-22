#![no_std]

mod swap_shuffle;
mod stride_shuffle;
mod perfect_riffle_shuffle;
mod bit_reverse_shuffle;
mod fold_shuffle;
mod multiplicative_shuffle;
mod utils;

#[derive(Debug,Clone,Copy,PartialEq,Eq,PartialOrd,Ord)]
pub struct Collection<T:Copy,const S:usize=256>{// For now collections of any (copy-able) type are legal.
    pub(crate)collection:Option<[T;S]>
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum ShuffleType{
    Swap(usize),Stride(usize),PerfectRiffle,BitReversal,Fold,Multiplicative(usize),
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

    pub const unsafe fn shuffle_unchecked(&mut self,shuffle_type:ShuffleType){
        *self=unsafe{self.shuffled_unchecked(shuffle_type)}
    }

    pub const fn shuffle(&mut self,shuffle_type:ShuffleType){
        *self=self.shuffled(shuffle_type)
    }

    pub const fn shuffle_soft(&mut self,shuffle_type:ShuffleType){
        *self=self.shuffled_soft(shuffle_type)
    }

    pub const unsafe fn shuffled_unchecked(&self,shuffle_type:ShuffleType)->Self{
        return match shuffle_type{
            ShuffleType::Swap(swap_offset)=>unsafe{self.swap_shuffled_unchecked(swap_offset)},
            ShuffleType::Stride(stride_step)=>unsafe{self.stride_shuffled_unchecked(stride_step)},
            ShuffleType::PerfectRiffle=>unsafe{self.perfect_riffle_shuffled_unchecked()},
            ShuffleType::BitReversal=>unsafe{self.bit_reversal_shuffled_unchecked()},
            ShuffleType::Fold=>unsafe{self.fold_shuffled_unchecked()},
            ShuffleType::Multiplicative(multiplier)=>unsafe{self.multiplicative_shuffled_unchecked(multiplier)},
            _=>*self
        }
    }

    pub const fn shuffled(&self,shuffle_type:ShuffleType)->Self{
        return match shuffle_type{
            ShuffleType::Swap(swap_offset)=>self.swap_shuffled(swap_offset),
            ShuffleType::Stride(stride_step)=>self.stride_shuffled(stride_step),
            ShuffleType::PerfectRiffle=>self.perfect_riffle_shuffled(),
            ShuffleType::BitReversal=>self.bit_reversal_shuffled(),
            ShuffleType::Fold=>self.fold_shuffled(),
            ShuffleType::Multiplicative(multiplier)=>self.multiplicative_shuffled(multiplier),
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
            ShuffleType::Multiplicative(multiplier)=>self.multiplicative_shuffled_soft(multiplier),
            _=>*self
        }
    }

    pub const unsafe fn shuffled_unchecked_repeated_unchecked(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        let mut collection:Collection<T,S>=*self;
        let mut repetitions:usize=repetitions;
        while repetitions>0{
            repetitions-=1;
            unsafe{collection.shuffle_unchecked(shuffle_type)}
        }
        collection
    }

    pub const unsafe fn shuffled_repeated_unchecked(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        let mut collection:Collection<T,S>=*self;
        let mut repetitions:usize=repetitions;
        while repetitions>0{
            repetitions-=1;
            collection.shuffle(shuffle_type)
        }
        collection
    }

    pub const unsafe fn shuffled_soft_repeated_unchecked(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        let mut collection:Collection<T,S>=*self;
        let mut repetitions:usize=repetitions;
        while repetitions>0{
            repetitions-=1;
            collection.shuffle_soft(shuffle_type)
        }
        collection
    }

    pub const unsafe fn shuffled_unchecked_repeated(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        if self.is_none()||repetitions==0{return Self{collection:None}}
        let mut collection:Collection<T,S>=*self;
        let mut repetitions:usize=repetitions;
        while repetitions>0{
            repetitions-=1;
            unsafe{collection.shuffle_unchecked(shuffle_type)}
        }
        collection
    }

    pub const fn shuffled_repeated(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        if self.is_none()||repetitions==0{return Self{collection:None}}
        let mut collection:Collection<T,S>=*self;
        let mut repetitions:usize=repetitions;
        while repetitions>0{
            repetitions-=1;
            collection.shuffle(shuffle_type)
        }
        collection
    }

    pub const fn shuffled_soft_repeated(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        if self.is_none()||repetitions==0{return Self{collection:None}}
        let mut collection:Collection<T,S>=*self;
        let mut repetitions:usize=repetitions;
        while repetitions>0{
            repetitions-=1;
            collection.shuffle_soft(shuffle_type)
        }
        collection
    }

    pub const unsafe fn shuffled_unchecked_repeated_soft(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        if self.is_none(){return Self{collection:None}}
        else if repetitions==0{return*self}
        let mut collection:Collection<T,S>=*self;
        if repetitions==1{unsafe{collection.shuffle_unchecked(shuffle_type)}}
        else{
            let mut repetitions:usize=repetitions;
            while repetitions>0{
                repetitions-=1;
                unsafe{collection.shuffle_unchecked(shuffle_type)}
            }
        }
        collection
    }

    pub const fn shuffled_repeated_soft(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        if self.is_none(){return Self{collection:None}}
        else if repetitions==0{return*self}
        let mut collection:Collection<T,S>=*self;
        if repetitions==1{collection.shuffle(shuffle_type)}
        else{
            let mut repetitions:usize=repetitions;
            while repetitions>0{
                repetitions-=1;
                collection.shuffle(shuffle_type)
            }
        }
        collection
    }

    pub const fn shuffled_soft_repeated_soft(&self,shuffle_type:ShuffleType,repetitions:usize)->Self{
        if self.is_none(){return Self{collection:None}}
        else if repetitions==0{return*self}
        let mut collection:Collection<T,S>=*self;
        if repetitions==1{collection.shuffle_soft(shuffle_type)}
        else{
            let mut repetitions:usize=repetitions;
            while repetitions>0{
                repetitions-=1;
                collection.shuffle_soft(shuffle_type)
            }
        }
        collection
    }

    pub const fn auto_shuffle(&mut self){
        *self=self.auto_shuffled();
    }

    pub const fn auto_shuffled(&self)->Self{
        if self.is_none(){return Self{collection:None}}
        else if S<2{return*self}

        let mut result_collection:Collection<T,S>=*self;
        if S==2{
            let mut rcl_array:[T;S]=result_collection.collection.unwrap();
            rcl_array.reverse();
            result_collection=Self{collection:Some(rcl_array)}
        }
        else{
            if S.is_power_of_two(){
                result_collection=unsafe{
                    result_collection
                    .swap_shuffled_unchecked(S/2-1)
                    .bit_reversal_shuffled_unchecked()
                    .perfect_riffle_shuffled_unchecked()
                    .swap_shuffled(S/3)
                }
            }
            else if S%2==0{
                result_collection=unsafe{
                    result_collection
                    .fold_shuffled_unchecked()
                    .swap_shuffled_unchecked(S/3)
                    .perfect_riffle_shuffled_unchecked()
                    .swap_shuffled_unchecked(S/5+1)
                }
            }
            else{
                result_collection=unsafe{
                    result_collection
                    .swap_shuffled_unchecked(S/2)
                    .stride_shuffled_unchecked(2)
                    .swap_shuffled_unchecked(2)
                    .fold_shuffled_soft()// For now.
                    .swap_shuffled_unchecked(S/5)
                }
            }
        }
        result_collection
    }
}