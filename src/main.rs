use crate::support::Dispatch;

mod balances;
mod support;
mod system;

mod types {
    use crate::support;

    pub type AccountId = String;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Balance = u128;

    pub type Extrinsic = support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = support::Header<BlockNumber>;
    pub type Block = support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall {
    BalanceTransfer { to: types::AccountId, amount: types::Balance }
}

impl system::Config for Runtime {
    type AccountId = String;
    type BlockNumber = u32;
    type Nonce = u32;
}

impl balances::Config for Runtime {
    type Balance = u128;
}

#[derive(Debug)]
pub struct Runtime {
    system: system::Pallet<Runtime>,
    balances: balances::Pallet<Runtime>,
}

impl support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;

    type Call = RuntimeCall;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> support::DispatchResult {
        match call {
            RuntimeCall::BalanceTransfer { to, amount } => {
                self.balances.transfer(&caller, &to, amount)?;
            }
        }
        Ok(())
    }
}

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        if self.system.block_number() != block.header.block_number {
            return Err("Block number mismatch");
        }

        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.system.inc_nonce(&caller);
            let _ = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }

        Ok(())
    }
}

fn main() {
    let mut runtime = Runtime::new();

    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();

    runtime.balances.set_balance(&alice, 100);
    runtime.system.inc_block_number();
    assert_eq!(runtime.system.block_number(), 1);

    runtime.system.inc_nonce(&alice);
    let _ = runtime
        .balances
        .transfer(&alice, &bob, 10)
        .map_err(|e| println!("Error: {:?}", e));

    runtime.system.inc_nonce(&alice);
    let _ = runtime
        .balances
        .transfer(&alice, &charlie, 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:#?}", runtime);
}
