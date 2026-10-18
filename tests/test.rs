use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a:Collection<i32,6>=Collection::from([1,2,3,4,5,6]);
    println!("{:?}",collection_a.shuffled(ShuffleType::Swap(collection_a.len()/2)).as_array())
}
