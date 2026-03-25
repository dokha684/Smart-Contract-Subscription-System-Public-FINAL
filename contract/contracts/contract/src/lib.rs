#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, Env, Address, Symbol, Vec
};

#[contracttype]
#[derive(Clone)]
pub struct Subscriber {
    pub expiry: u64,
    pub active: bool,
}

#[contract]
pub struct SubscriptionContract;

#[contractimpl]
impl SubscriptionContract {

    // Initialize contract
    pub fn init(env: Env, owner: Address, fee: i128, duration: u64) {
        owner.require_auth();

        env.storage().instance().set(&Symbol::short("OWNER"), &owner);
        env.storage().instance().set(&Symbol::short("FEE"), &fee);
        env.storage().instance().set(&Symbol::short("DUR"), &duration);
    }

    // Subscribe
    pub fn subscribe(env: Env, user: Address) {
        user.require_auth();

        let fee: i128 = env.storage().instance().get(&Symbol::short("FEE")).unwrap();
        let duration: u64 = env.storage().instance().get(&Symbol::short("DUR")).unwrap();

        let mut sub: Subscriber = env.storage().persistent()
            .get(&user)
            .unwrap_or(Subscriber { expiry: 0, active: false });

        let now = env.ledger().timestamp();

        let new_expiry = if sub.active && sub.expiry > now {
            sub.expiry + duration
        } else {
            now + duration
        };

        sub.expiry = new_expiry;
        sub.active = true;

        env.storage().persistent().set(&user, &sub);

        // (Optional) Transfer token logic can be added here
    }

    // Renew
    pub fn renew(env: Env, user: Address) {
        user.require_auth();

        let duration: u64 = env.storage().instance().get(&Symbol::short("DUR")).unwrap();

        let mut sub: Subscriber = env.storage().persistent()
            .get(&user)
            .expect("Not subscribed");

        if !sub.active {
            panic!("Inactive");
        }

        sub.expiry += duration;

        env.storage().persistent().set(&user, &sub);
    }

    // Cancel
    pub fn cancel(env: Env, user: Address) {
        user.require_auth();

        let mut sub: Subscriber = env.storage().persistent()
            .get(&user)
            .expect("Not subscribed");

        sub.active = false;

        env.storage().persistent().set(&user, &sub);
    }

    // Check active
    pub fn is_active(env: Env, user: Address) -> bool {
        let sub: Subscriber = env.storage().persistent()
            .get(&user)
            .unwrap_or(Subscriber { expiry: 0, active: false });

        let now = env.ledger().timestamp();

        sub.active && sub.expiry > now
    }

    // Withdraw (basic version)
    pub fn get_owner(env: Env) -> Address {
        env.storage().instance().get(&Symbol::short("OWNER")).unwrap()
    }

    pub fn get_fee(env: Env) -> i128 {
        env.storage().instance().get(&Symbol::short("FEE")).unwrap()
    }

    pub fn get_duration(env: Env) -> u64 {
        env.storage().instance().get(&Symbol::short("DUR")).unwrap()
    }
}
stellar contract invoke \
  --id CDA3AYDI35SBO2YG63SYHDLP5NB2FLZATPSMMFQI3EZC4CMSZR5ATPM3 \
  --source student \
  --network testnet \
  -- init \
  --owner student \
  --fee 1000000 \
  --duration 60