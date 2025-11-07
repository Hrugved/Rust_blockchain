use std::collections::BTreeMap;

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String,u32>, // wallet to #transactions
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            block_number: 0,
            nonce: BTreeMap::new()
        }
    }

    pub fn inc_block_number(&mut self) {
        self.block_number = self.block_number.checked_add(1).expect("Blockchain just crashed due to overflow")
    }

    pub fn inc_nonce(&mut self, who: &String) {
        let curr_nonce = self.nonce.get(who).unwrap_or(&0);
        let new_nonce = curr_nonce.checked_add(1).unwrap_or_else(|| panic!("Nonce overflow for {}", who));
        self.nonce.insert(who.clone(), new_nonce);
    }

    pub fn get_nonce(&self, who: &String) -> u32 {
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
        assert_eq!(system.get_nonce(alice), 1);
    }

}
