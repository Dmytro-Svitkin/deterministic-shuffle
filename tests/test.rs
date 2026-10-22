use deterministic_shuffle::*;

#[test]
fn test(){
    let collection_a:Collection<u16,4094>=generate_consecutive_collection();
    println!("{:?}",unsafe{collection_a.shuffled_unchecked_repeated_unchecked(ShuffleType::PerfectRiffle,7).fold_shuffled().perfect_riffle_shuffled().stride_shuffled(7).perfect_riffle_shuffled()})
}

fn generate_consecutive_collection<const N:usize>()->Collection<u16,N>{
    let mut result_array:[u16;N]=[0u16;N];
    for ix in 1..=N{
        result_array[ix-1]=ix as u16
    }
    return Collection::from(result_array)
}