use tiny_keccak::{Hasher, Keccak};

// hash a string
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(data);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    return output;
}

pub struct MerkleTree {
    pub layers: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    pub fn new(items: Vec<&str>) -> Self {
        let mut leaves: Vec<[u8; 32]> = items.iter().map(|i| keccak256(i.as_bytes())).collect();
        // Sort leaves for verification easier.
        leaves.sort();
        Self {
            layers: MerkleTree::build_tree(leaves),
        }
    }

    pub fn merkle_root(&self) -> [u8; 32] {
        return self.layers[0][0];
    }

    // For a given height, get the number of nodes in the tree
    fn get_tree_nodes(height: usize) -> usize {
        return (2usize).pow((height).try_into().unwrap()) - 1;
    }

    // Given the number of leaves, get the height of the tree, height = log2(2*leaves - 1).
    fn get_tree_height(leaves: usize) -> usize {
        let count = (2 * leaves - 1) as f32;
        return count.log2().floor() as usize;
    }

    // Returns a merkle proof for the given item.
    pub fn merkle_proof(&self, item: &str) -> Vec<[u8; 32]> {
        let mut proof = Vec::<[u8; 32]>::new();
        let leaf = keccak256(item.as_bytes());
        let leaves = self.layers.last().unwrap();
        let leaf_index = leaves.iter().position(|e| e == &leaf).unwrap();

        // Current index used for traversal, represents the index in the entire tree and not in the layer
        let mut current_index = MerkleTree::get_tree_nodes(self.layers.len() - 1) + leaf_index;

        for layer_index in (1..self.layers.len()).rev() {
            let layer = &self.layers[layer_index];
            // Internal index represents the index in the current layer
            let internal_index = current_index - MerkleTree::get_tree_nodes(layer_index);
            let sibling = if internal_index % 2 == 0 {
                // Point to last element in the layer, the sibling is itself.
                if internal_index == layer.len() - 1 {
                    layer[internal_index]
                } else {
                    layer[internal_index + 1]
                }
            } else {
                layer[internal_index - 1]
            };
            proof.push(sibling);
            // Update current index to the parent of the current index.
            current_index = (current_index - 1) / 2;
        }
        return proof;
    }

    // Construct a full merkle tree, layers size == height + 1.
    fn build_tree(leaves: Vec<[u8; 32]>) -> Vec<Vec<[u8; 32]>> {
        let height = MerkleTree::get_tree_height(leaves.len());
        // Initialize the layers, each layer is a vector of nodes. Layer size is 1 and only store the leaves at first.
        let mut layers = vec![leaves.to_vec()];
        for layer_index in 1..height + 1 {
            let mut layer = Vec::<[u8; 32]>::new();
            let previous_layer = &layers[layer_index - 1];
            for i in (0..previous_layer.len()).step_by(2) {
                let left = previous_layer[i];
                let right = if i + 1 < previous_layer.len() {
                    previous_layer[i + 1]
                } else {
                    // Duplicate hash in case of odd number of leaves / unbalanced tree
                    left
                };
                let mut combined = [left, right];
                // Sort pairs for easier verification
                combined.sort();
                layer.push(keccak256(&combined.concat()));
            }
            layers.push(layer);
        }
        // Reverse to get root at the layer one
        layers.reverse();
        return layers;
    }

    /* Verify proof for a given item.
        Hash the given leaf and each of the sibling in the proof, then verify if root hash is equal or not.
    */
    pub fn verify_proof(root: [u8; 32], proof: Vec<[u8; 32]>, leaf: [u8; 32]) -> bool {
        let mut current = leaf;
        for elem in proof {
            current = if current <= elem {
                keccak256(&[current, elem].concat())
            } else {
                keccak256(&[elem, current].concat())
            }
        }
        return current == root;
    }
}

#[cfg(test)]
mod tests {

    use merkle_tree::{keccak256, MerkleTree};

    #[test]
    fn test_merkle_proof_1() {
        let items = ["foo", "bar", "baz", "qux"];
        let tree = MerkleTree::new(items.to_vec());

        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 2);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_merkle_proof_2() {
        let items = ["foo", "bar", "baz", "qux", "quux", "quuux"];
        let tree = MerkleTree::new(items.to_vec());

        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 3);
            assert_eq!(result, true);
        }
    }

    #[test]
    fn test_merkle_proof_3() {
        let items = ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"];
        let tree = MerkleTree::new(items.to_vec());

        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 4);
            assert_eq!(result, true);
        }
    }

    // home problem dataset.
    #[test]
    fn test_merkle_proof_4() {
        let items = ["abc", "bcd", "cde", "def", "efg"];
        let tree = MerkleTree::new(items.to_vec());
        for i in 0..items.len() {
            let proof = tree.merkle_proof(items[i]);
            let length = proof.len();
            let leaf = keccak256(items[i].as_bytes());
            let result = MerkleTree::verify_proof(tree.merkle_root(), proof, leaf);
            assert_eq!(length, 3);
            assert_eq!(result, true);
        }
    }
}
