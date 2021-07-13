#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;
use ink_prelude::vec::Vec;
pub use fantour_contract::*;

#[ink::contract(env = CustomEnvironment)]
mod contract_demo {
    use super::*;

    #[cfg(not(feature = "ink-as-dependency"))]
    #[ink(storage)]
    pub struct ContractDemo {
        value: [u8; 32],
    }

    #[ink(event)]
    pub struct RandomUpdated {
        #[ink(topic)]
        new: [u8; 32],
    }

    #[ink(event)]
    pub struct CreateClassFromContract {
        #[ink(topic)]
        owner: AccountId,
        class_id: ClassId,
    }

    impl ContractDemo {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { value: Default::default() }
        }

        /// a handy test case:
        /// Account: //Alice
        /// signature: 0x00277901dacb28bf5f34f172d065a27d9ab97f231bf799f6beee4b0a4d7d702acbfd1c9c155517efd6ad71bb280a0fccfd33e0b3517c06f07ca1fcd7fec32888
        /// message: dddddddaaaa1
        /// return: true
        #[ink(message)]
        pub fn sr25519_verify(&self, account: AccountId, signature: Vec<u8>, message: Vec<u8>) -> bool {
            self.env().extension().sr25519_verify(&account, signature, message)
        }

        #[ink(message)]
        pub fn tokens(&self, class_id: ClassId, token_id: TokenId) -> (Metadata, Quantity, BlockNumber) {
            let info: Option<ContractTokenInfo<_, _, _, _, _>> = self.env().extension().tokens(class_id, token_id);
            let info = info.unwrap_or_default();
            (info.metadata, info.quantity, info.data.create_block)
        }

        #[ink(message)]
        pub fn create_class(
            &mut self,
            metadata: Metadata,
            name: Chars,
            description: Chars,
            properties: u8,
        ) -> Result<(), FantourErr> {
            let (owner, class_id) = self.env().extension().create_class(metadata, name, description, properties)?;
            self.env().emit_event(CreateClassFromContract { owner, class_id });
            Ok(())
        }

        #[ink(message)]
        pub fn mint_nft(
            &mut self,
            class_id: ClassId,
            metadata: Metadata,
            quantity: Quantity,
            charge_royalty: Option<bool>,
        ) -> Result<(), FantourErr> {
            let (_class_owner, _beneficiary, _class_id, _token_id, _quantity) = self.env().extension().proxy_mint(
                &self.env().caller(),
                class_id,
                metadata,
                quantity,
                charge_royalty,
            )?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(
            &mut self,
            to: AccountId,
            class_id: ClassId,
            token_id: TokenId,
            quantity: Quantity,
        ) -> Result<(), FantourErr> {
            self.env().extension().transfer(&to, class_id, token_id, quantity)?;
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_all(
            &mut self,
            to: AccountId,
            items: Vec<(ClassId, TokenId, Quantity)>,
        ) -> Result<(), FantourErr> {
            for (class_id, token_id, quantity) in items {
                self.env().extension().transfer(&to, class_id, token_id, quantity)?;
            }
            Ok(())
        }

        #[ink(message)]
        pub fn update(&mut self) -> Result<(), FantourErr> {
            let new_random = self.env().extension().fetch_random()?;
            self.value = new_random;
            self.env().emit_event(RandomUpdated { new: new_random });
            Ok(())
        }

        #[ink(message)]
        pub fn get(&self) -> [u8; 32] {
            self.value
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn new_works() {
            let _contract = ContractDemo::new();
        }
    }
}
