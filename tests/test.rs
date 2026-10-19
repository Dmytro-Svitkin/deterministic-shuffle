use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a:Collection<i32,0>=Collection::from([]);
    println!("{:?}",collection_a.stride_shuffled(0))
}
