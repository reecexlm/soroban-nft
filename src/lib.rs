#![no_std]
use soroban_sdk::{contractimpl, contracttype, storage, symbol, log, vec, Env, Vec, map, Map, AccountId, Address};
use soroban_sdk::Symbol;

const COUNTER: Symbol = symbol!("COUNTER");

pub struct IncrementContract;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StellarNft {
    pub nft_id: u32,
    pub url: Option<Symbol>,
    pub owner: Option<Address>,
}


impl Default for StellarNft {
    fn default() -> Self {
        StellarNft {
            nft_id: 0,
            url: None,
            owner: None
        }
    }
}

pub struct StellarContract;

#[contractimpl]
impl StellarContract {

    pub fn transfer(env: Env, token_id: u64, transfer_to: Address) {
        // Get the current owner of the token
        //let token_account: Address = Address::Account(account_id);
        
        let mut token: StellarNft = env.storage()
            .get(token_id)
            .unwrap_or_else(|| Ok(StellarNft::default())) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not a State. 
            token.owner = Some(transfer_to);
        env.storage().set(token_id, &token);
    }


    pub fn mint(env: Env) -> StellarNft {
        let mut count: u32 = env
        .storage()
        .get(COUNTER)
        .unwrap_or(Ok(0)) // If no value set, assume 0.
        .unwrap(); // Panic if the value of COUNTER is not u32.

        // Increment the count.
        count += 1;

        // Save the count.
        env.storage().set(COUNTER, count);
        let nft_id = count;
        let mut nft = Self::get_nft(env.clone(), nft_id);
        nft.nft_id = nft_id;
        nft.url = Some(symbol!("4vucchjy"));
        env.storage().set(nft_id, &nft);
        nft
    }

    pub fn get_nft(env: Env, nft_id: u32) -> StellarNft {
        env.storage()
            .get(nft_id)
            .unwrap_or_else(|| Ok(StellarNft::default())) // If no value set, assume 0.
            .unwrap() // Panic if the value of COUNTER is not a State.
    }

    pub fn get_url(env: Env, nft_id: u32) -> StellarNft {
        env.storage()
            .get(nft_id)
            .unwrap_or_else(|| Ok(StellarNft::default())) // If no value set, assume 0.
            .unwrap() // Panic if the value of COUNTER is not a State.
    }

    fn set_url(env: Env, token_id: u64, url: Symbol) {
        // Get the current owner of the token
        let mut token: StellarNft = env.storage()
            .get(token_id)
            .unwrap_or_else(|| Ok(StellarNft::default())) // If no value set, assume 0.
            .unwrap(); // Panic if the value of COUNTER is not a State.
        token.url = Some(url);
        env.storage().set(token_id, &token);
    } 
}