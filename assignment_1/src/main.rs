use sha2::{Digest,Sha256};

pub trait SumCommitment{

    fn amount(&self) -> u64;                // returns the amount of bobcoin
    fn digest(&self) -> [u8; 32];           // returns the hash
}

pub trait ExclusiveAllotmentProof<C: SumCommitment>{
    fn position(&self) -> usize;                        //returns the position
    fn sibling(&self,height : u8) -> Option <C >;       //returns the sibling
    fn verify(&self , root_commitment: &C) -> bool;     //verifies that the root equals to 
}

pub trait MerkleTree<C: SumCommitment,P: ExclusiveAllotmentProof<C>>{
    fn new (values: Vec<u64>) -> Self;              // returns the whole array of arrays of the tree
    fn commit(&self) -> C;                          // returns
    fn prove(&self, position:usize) -> P;           // initial step of verify :D  
}

fn hash_bytes (slice: &[u8]) -> [u8; 32]{
    let mut hasher = Sha256::new();
    hasher.update(slice);
    hasher.finalize().into()
}


//1) Implement a structure that implements the ‘SumCommitment‘ trait.

#[derive(Clone)]
pub struct SumCommitNode{                          
    amount:u64,                     
    digest:[u8;32]                 
}
impl SumCommitment for SumCommitNode{
    fn amount(&self) -> u64{
        self.amount                 
    }
    fn digest(&self) -> [u8; 32]{
        self.digest                
    }
}





impl ExclusiveAllotmentProof<SumCommitNode> for AllotmentProof{
    fn position(&self) -> usize{
        self.pos                                                        
    }
fn sibling(&self, height: u8) -> Option<SumCommitNode> {
    // Ensure the height is within valid range.
    if height as usize >= self.vv.len() {
        return None;
    }
    // Calculate the position of the current node in the tree.
    let my_pos = self.pos / ((height as usize) + 1);
      // Check if we are looking at the last level of the tree.
    if height as usize == self.vv.len() - 1 {
        return Some(self.vv[height as usize][my_pos].clone());
    }
        // Determine the position of the sibling.
    // If the current node's position is even, then the sibling is the next node.
    // Otherwise, the sibling is the previous node.
    let sibling_pos = if my_pos % 2 == 0 {
        my_pos + 1
    } else {
        my_pos - 1
    };
    // Check if the computed sibling position is within the bounds of the tree at the given height.
    if sibling_pos < self.vv[height as usize].len() {
        return Some(self.vv[height as usize][sibling_pos].clone());
    }
     // If we've reached here, the sibling position is out of bounds, so return None.
    None
}

    fn verify(&self , root_commitment: &SumCommitNode) -> bool{                   // proves that the leaf belongs to the given tree root  
       
       let  my_pos= self.pos;
        let my_limit = self.vv.len();
      let  mut curheight = 0; 
     let mut parent_hash = [0; 32];
        
 // Loop through the tree to the root.
         while curheight<my_limit-1
       {
           let  my_sibling = self.sibling(curheight as u8).unwrap();
       let left_hash;
       let right_hash;
       if my_pos%2==0 {
                 left_hash = self.vv[curheight  as usize][my_pos ].digest;
             right_hash = my_sibling.digest;
        }
        else
        {
              right_hash = self.vv[curheight  as usize][my_pos ].digest;
             left_hash = my_sibling.digest;
        }
         parent_hash = hash_bytes(&[left_hash, right_hash].concat());
         
        curheight+=1;
       }
       return root_commitment.digest == parent_hash; // Return true if the last combined hash matches the tree root.
    }
}

struct MyStruct {
    vv: Vec<Vec<SumCommitNode>>         // Multi-dimensional vector representing the Merkle tree.
}

struct AllotmentProof {
     vv: Vec<Vec<SumCommitNode>>,           // Multi-dimensional vector representing the Merkle tree.
     pos : usize                            // postion
}

impl MerkleTree<SumCommitNode,AllotmentProof> for MyStruct {   
     // Creates a new Merkle tree from a list of values.
    fn new (values: Vec<u64>) -> Self {
      let len = values.len();
    let mut num_columns = len;

    let mut rows = Vec::new();

    while num_columns > 0 {
        rows.push(vec![SumCommitNode { amount: 0, digest: [0; 32] }; num_columns]);
        num_columns /= 2;
       
    }

    let mut result: Vec<Vec<SumCommitNode>> = rows;
    

    // Fill the first row with 'values'
    for (i, &value) in values.iter().enumerate() {
        let hash = hash_bytes(&value.to_le_bytes());
        result[0][i] = SumCommitNode { amount: value, digest:hash };
    }

    let mut current_row = 1;
    let mut current_col = 0;
    let mut len = len / 2;

    while len > 0 {         // Iterate while there are still rows in the tree to be processed.
        while current_col <= len / 2 {              // Extract the digest (hash) of the left and right child nodes.
            let left_hash = result[current_row - 1][current_col * 2].digest;
            let right_hash = result[current_row - 1][current_col * 2 + 1].digest;

            let parent_hash = hash_bytes(&[left_hash, right_hash].concat());
            let cur_amount =
                result[current_row - 1][current_col * 2].amount + result[current_row - 1][current_col * 2 + 1].amount;
        // Set the current node in the result to be a new SumCommitNode 
        // with the combined amount and the computed hash.
            result[current_row][current_col] = SumCommitNode {
                amount: cur_amount,
                digest: parent_hash,
            };
            current_col += 1;
        }
        // Halve the length since each level up in the tree reduces 
    // the number of nodes by half.
        len /= 2;
         // Move to the next row and reset the column index to start.
        current_row += 1;
        current_col = 0;
    }
     MyStruct { vv: result }
}
fn commit(&self) -> SumCommitNode {                         // returns the root
    let size = self.vv.len();
    self.vv[size - 1][0].clone()
}
fn prove(&self, position:usize) ->AllotmentProof {          
    let result = AllotmentProof{            //generates a proof of work for the given leaf (transaction)
        vv:self.vv.clone(),
        pos : position
    };
    return result;
}                 

}   
fn main() {
    let values = vec![1, 2, 3, 4];      //testing the trees 
    
   let values2 = vec![2, 2, 3, 4]; 

    let tree = MyStruct::new(values);
    let prove_me_right = tree.prove(0);



      let tree2 = MyStruct::new(values2);
       let prove_me_wrong = tree2.prove(0);
      
    println!("it must be right {}", prove_me_right.verify(&tree.commit()));
    println!("it must be false {}", prove_me_wrong.verify(&tree.commit()));

  
    for (row_index, row) in tree.vv.iter().enumerate() {
        for (col_index, node) in row.iter().enumerate() {
            println!("Row {} - Col {}: Amount = {}, Hash = {:?}", row_index, col_index, node.amount, node.digest);
        }
    }
    
    let root_commitment = tree.commit();

    println!("Root Commitment: Amount = {}, Digest = {:?}", root_commitment.amount, root_commitment.digest);
}