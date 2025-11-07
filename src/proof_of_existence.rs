use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::{self, DispatchResult};

pub trait Config: crate::system::Config {
    type Content: Debug + Ord;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        match self.get_claim(&claim) {
            Some(_) => Err("Claim already exists"),
            None => {
                self.claims.insert(claim, caller);
                Ok(())
            } 
        }
    }

    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let claim_owner = self.claims.get(&claim).ok_or("Claim does not exists")?;
        if claim_owner != &caller {
            return Err("Caller is not the owner of claim");
        }
        self.claims.remove(&claim);
        Ok(())
    }

}


pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
}

impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;

    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim),
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::system;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    
    #[test]
    fn basic_proof_of_existence() {
        let mut poe: super::Pallet<TestConfig> = super::Pallet::new();
        let _ = poe.create_claim("alice".to_string(), "alice_claim");
        assert_eq!(poe.get_claim(&"alice_claim"), Some(&"alice".to_string()));
        
        let res = poe.revoke_claim("bob".to_string(), "alice_claim");
        assert_eq!(res, Err("Caller is not the owner of claim"));

        let res = poe.create_claim("bob".to_string(), "alice_claim");
        assert_eq!(res, Err("Claim already exists"));

        let res = poe.revoke_claim("alice".to_string(), "non existent claim");
        assert_eq!(res, Err("Claim does not exists"));

        let res = poe.revoke_claim("alice".to_string(), "alice_claim");
        assert_eq!(res, Ok(()));
        assert_eq!(poe.get_claim(&"alice_claim"), None);
    }

}
