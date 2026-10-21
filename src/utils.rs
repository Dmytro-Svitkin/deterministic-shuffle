#[inline(always)]
pub(crate)const fn reverse_bits(mut value:usize,anchor_value_bits:u32)->usize{
    let mut result:usize=0;
    let mut counter:u32=0;
    while counter<anchor_value_bits{
        result=(result<<1)|(value&1);
        value>>=1;
        counter+=1
    }
    result
}

#[inline(always)]
pub(crate)const fn gcd(mut a:usize,mut b:usize)->usize{
    while b!=0{
        let rcl:usize=b;
        b=a%b;
        a=rcl;
    }
    if a==0{return 1}
    a
}