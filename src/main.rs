mod core;

fn main() {
    let previous_hash = [120, 184, 118, 89, 133, 231, 89, 138, 66, 99, 53, 131, 79, 207, 62, 81, 223, 161, 47, 153, 18, 85, 21, 88, 134, 54, 1, 30, 143, 160, 204, 77]; 
    let transactions = vec![1, 2, 3, 4];
    let block_one = core::block::Block::new(1, previous_hash, transactions); // maybe make this a normal function like new_block instead

    println!("{:?}", block_one);
    let result = block_one.hash().unwrap();
    
    println!("{:?}", result);

    println!("{:?}", block_one);
}
