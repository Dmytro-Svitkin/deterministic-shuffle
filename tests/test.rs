use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a:Collection<i32,0>=Collection::from([]);
    println!("{:?}",collection_a.bit_reversal_shuffled());
}
