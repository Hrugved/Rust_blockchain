mod balances;
mod system;

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

impl Runtime {
    fn new() -> Self {
        Self {
            system: system::Pallet::new(),
            balances: balances::Pallet::new(),
        }
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
    let _ = runtime.balances.transfer(&alice, &bob, 10)
        .map_err(|e| println!("Error: {:?}", e));

    runtime.system.inc_nonce(&alice);
    let _ = runtime.balances.transfer(&alice, &charlie, 20)
        .map_err(|e| println!("Error: {:?}", e));

    println!("{:#?}", runtime);


}
 