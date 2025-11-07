use std::collections::BTreeMap;

type AccountId = String;
type BlockNumber = u32;
type Nonce = u32;

#[derive(Debug)]
pub struct Pallet {
    block_number: BlockNumber,
    nonce: BTreeMap<String, Nonce>, // wallet to #transactions
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self
            .block_number
            .checked_add(1)
            .expect("Blockchain just crashed due to overflow")
    }

    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn inc_nonce(&mut self, who: &AccountId) {
        let curr_nonce = self.nonce.get(who).unwrap_or(&0);
        let new_nonce = curr_nonce
            .checked_add(1)
            .unwrap_or_else(|| panic!("Nonce overflow for {}", who));
        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn nonce(&self, who: &AccountId) -> Nonce {
        *self.nonce.get(who).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn init_sytem() {
        let system = super::Pallet::new();
        assert_eq!(system.block_number, 0)
    }

    #[test]
    fn inc_block_number() {
        let mut system = super::Pallet::new();
        system.inc_block_number();
        assert_eq!(system.block_number, 1);
    }

    #[test]
    fn inc_nonce() {
        let mut system = super::Pallet::new();
        let alice = &"alice".to_string();
        system.inc_nonce(alice);
        assert_eq!(system.nonce(alice), 1);
    }
}
