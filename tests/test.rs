use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a:Collection<i32,1>=Collection::from([12]);
    println!("{:?}",collection_a.stride_shuffled_soft(1))
}
