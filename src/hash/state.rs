use std::collections::hash_map::{DefaultHasher, RandomState};
use std::hash::BuildHasher;

use lazy_static::lazy_static;

lazy_static! {
    static ref HASHER: DefaultHasher = RandomState::default().build_hasher();
}

/// Allows for  hash sets  and maps  to be  composed, and  shared over
/// thread boundaries without breaking the Hash and Eq contracts.
///
/// One nice feature of immutable data structure is to be able to have
/// consistent  hashing   and  do   structural  sharing   over  thread
/// boundaries. Also being able to compose the hash maps and sets is a
/// nice feature. (E.g. a hash map in a set, or a set as a key).
///
/// Because rust  has a  random state hasher,  which is  different for
/// each  hash map  to  prevent crafted  key DOS  attacks.  It is  not
/// possible  to combine  hashed  collections like  the  set and  map,
/// because their hashes will be different.
///
/// This enables  a compromise. It  will have  a random state  on each
/// execution  of  the  process.  So different  executions  will  have
/// different  hashes.  But  the  hashes  are  consistent  within  one
/// process.  Which  enables composition of  maps and sets as  if they
/// are constant values.
///
/// Another workaround is to use OrdMap and OrdSet exclusively.
#[derive(Default)]
pub struct ConsistentState;

impl BuildHasher for ConsistentState {
    type Hasher = DefaultHasher;
    #[inline]
    #[allow(deprecated)]
    fn build_hasher(&self) -> DefaultHasher {
        HASHER.clone()
    }
}
