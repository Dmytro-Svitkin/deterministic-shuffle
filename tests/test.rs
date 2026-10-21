use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a=Collection::from([0,1,2,3,4,5,6,7]);
    println!("{:?}",collection_a.shuffled_repeated(ShuffleType::BitReversal,2))
}
