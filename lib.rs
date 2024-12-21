#![cfg_attr(not(feature = "std"), no_std, no_main)]

pub use openbrush::traits::{AccountId, Storage};

#[ink::storage_item]
#[openbrush::accessors(HatedStorageAccessors)]
#[derive(Debug)]
pub struct HatedStorage {
    #[get]
    #[set]
    pub hated_account: AccountId,
}

#[openbrush::implementation(PSP22, PSP22Burnable, PSP22Metadata, PSP22Mintable)]
#[openbrush::contract]
pub mod my_psp22 {
    use crate::*;
    use openbrush::{contracts::psp22::extensions::metadata, traits::String};

    #[ink(storage)]
    #[derive(Storage)]
    pub struct Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        hated_storage: HatedStorage,
        #[storage_field]
        metadata: metadata::Data,
    }

    #[overrider(psp22::PSP22Transfer)]
    fn _before_token_transfer(
        &mut self,
        _from: Option<&AccountId>,
        to: Option<&AccountId>,
        _amount: &Balance,
    ) -> Result<(), PSP22Error> {
        if _to == Some(&self.hated_storage.hated_account) {
            return Err(PSP22Error::Custom(String::from("I hate this account!")));
        }
        Ok(())
    }

    impl HatedStorageAccessors for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: String, symbol: String, decimal: u8) -> Self {
            let caller = Self::env().caller();
            let mut instance = Self {
                psp22: Default::default(),
                hated_storage: HatedStorage {
                    hated_account: [255; 32].into(),
                },
                metadata: Default::default(),
            };
            instance.metadata.name.set(&Some(name));
            instance.metadata.symbol.set(&Some(symbol));
            instance.metadata.decimals.set(&decimal);
            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint");
            instance
        }
    }
}

