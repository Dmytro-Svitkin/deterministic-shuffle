use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a:Collection<i32,16>=Collection::from([0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15]);
    println!("{:?}",collection_a.multiplicative_shuffled_soft(3));
}
