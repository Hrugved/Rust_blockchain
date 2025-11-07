use std::{collections::BTreeMap, fmt::Display};

use num::{CheckedAdd, CheckedSub, Zero, One};

pub trait Config {
    type AccountId: Ord + Clone + Display;
    type BlockNumber: Zero + One + CheckedSub + CheckedAdd + Copy;
    type Nonce: Zero + One + Ord + Clone + Copy + CheckedAdd;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccountId, T::Nonce>, // wallet to #transactions
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(&T::BlockNumber::one())
            .expect("Blockchain just crashed due to overflow")
    }

    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let curr_nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = curr_nonce
            .checked_add(&T::Nonce::one())
            .unwrap_or_else(|| panic!("Nonce overflow for {}", who));
        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
}

#[cfg(test)]
mod tests {

    struct TestConfig;

    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_sytem() {
        let system: super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(system.block_number, 0)
    }

    #[test]
    fn inc_block_number() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn inc_nonce() {
        let mut system: super::Pallet<TestConfig> = super::Pallet::new();
        let alice = &"alice".to_string();
        system.inc_nonce(alice);
        assert_eq!(system.nonce(alice), 1);
    }
}
 