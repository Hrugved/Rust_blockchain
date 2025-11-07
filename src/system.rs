use std::{collections::BTreeMap, fmt::Display};

use num::{CheckedAdd, CheckedSub, Zero, One};

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet<AccountId, BlockNumber, Nonce> {
    block_number: BlockNumber,
    nonce: BTreeMap<AccountId, Nonce>, // wallet to #transactions
}

impl<AccountId, BlockNumber, Nonce> Pallet<AccountId, BlockNumber, Nonce> 
where 
    AccountId: Ord + Clone + Display,
    BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy,
    Nonce: Zero + One + Ord + Clone + Copy + CheckedAdd,
{
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&BlockNumber::one())
            .expect("Blockchain just crashed due to overflow")
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let curr_nonce = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        let new_nonce = curr_nonce
            .checked_add(&Nonce::one())
            .unwrap_or_else(|| panic!("Nonce overflow for {}", who));
        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&Nonce::zero())
    }
}

#[cfg(test)]
mod tests {
    use crate::system::Pallet;


    #[test]
    fn init_sytem() {
        let system: super::Pallet<String, u32, u32> = super::Pallet::new();
        assert_eq!(system.block_number, 0)
    }

    #[test]
    fn inc_block_number() {
        let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn inc_nonce() {
        let mut system: super::Pallet<String, u32, u32> = super::Pallet::new();
        let alice = &"alice".to_string();
        system.inc_nonce(alice);
        assert_eq!(system.nonce(alice), 1);
    }
}
 