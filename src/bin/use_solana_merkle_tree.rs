use solana_merkle_tree::*;
use solana_program::hash::hashv;

const LEAF_PREFIX: &[u8] = &[0];
macro_rules! hash_leaf {
    {$d:ident} => {
        hashv(&[LEAF_PREFIX, $d])
    }
}

fn main() {
    let data = vec![b"example1", b"example2"];
    // Initialize a new Merkle tree
    let tree = MerkleTree::new(&data);

    // Generate the Merkle root
    let root = tree.get_root();

    // Generate a proof for a specific leaf (e.g., "example1")
    let leaf = b"example1";
    let proof = hash_leaf!(leaf);

    // Verify the proof
    let try_to_find = tree.find_path(0).unwrap();

    let is_valid = try_to_find.verify(proof);
    println!("Is valid: {:?}", is_valid);

    // Output the result
    println!("Merkle root: {:?}", root);
    println!("Proof: {:?}", proof);
}
