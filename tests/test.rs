use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a:Collection<i32,10>=Collection::from([0,1,2,3,4,5,6,7,8,9]);
    println!("{:?}",unsafe{collection_a.perfect_riffle_shuffled_unchecked()})
}
