#[cfg(all(test, feature = "e2e-tests"))]
pub mod tests {
    use crate::hatedstorageaccessors_external::HatedStorageAccessors;
    use ink_e2e::{build_message, PolkadotConfig};
    use my_psp22::my_psp22::*;
    use openbrush::contracts::psp22::psp22_external::PSP22;
    use test_helpers::{address_of, balance_of};

    type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

    #[ink_e2e::test]
    async fn assigns_initial_balance(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let constructor = ContractRef::new(100);
        let address = client
            .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let result = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.balance_of(address_of!(Alice)));
            client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
        };

        assert!(matches!(result.return_value(), 100));

        Ok(())
    }

    #[ink_e2e::test]
    async fn transfer_adds_amount_to_destination_account(
        mut client: ink_e2e::Client<C, E>,
    ) -> E2EResult<()> {
        let constructor = ContractRef::new(100);
        let address = client
            .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let result = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.transfer(address_of!(Bob), 50, vec![]));
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("transfer failed")
        };

        assert!(matches!(result.return_value(), Ok(())));

        let balance_of_alice = balance_of!(client, address, Alice);

        let balance_of_bob = balance_of!(client, address, Bob);

        assert_eq!(balance_of_bob, 50, "Bob should have 50 tokens");
        assert_eq!(balance_of_alice, 50, "Alice should have 50 tokens");

        Ok(())
    }

    #[ink_e2e::test]
    async fn cannot_transfer_above_the_amount(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let constructor = ContractRef::new(100);
        let address = client
            .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let result = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.transfer(address_of!(Bob), 101, vec![]));
            client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
        };

        assert!(matches!(
            result.return_value(),
            Err(PSP22Error::InsufficientBalance)
        ));

        Ok(())
    }

    #[ink_e2e::test]
    async fn cannot_transfer_to_hated_account(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
        let constructor = ContractRef::new(100);
        let address = client
            .instantiate("my_psp22", &ink_e2e::alice(), constructor, 0, None)
            .await
            .expect("instantiate failed")
            .account_id;

        let result = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.transfer(address_of!(Bob), 10, vec![]));
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("transfer failed")
        };

        assert!(matches!(result.return_value(), Ok(())));

        let balance_of_bob = balance_of!(client, address, Bob);

        assert!(matches!(balance_of_bob, 10));

        let result = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.set_hated_account(address_of!(Bob)));
            client
                .call(&ink_e2e::alice(), _msg, 0, None)
                .await
                .expect("set_hated_account failed")
        };

        assert!(matches!(result.return_value(), ()));

        let result = {
            let _msg = build_message::<ContractRef>(address.clone())
                .call(|contract| contract.transfer(address_of!(Bob), 10, vec![]));
            client.call_dry_run(&ink_e2e::alice(), &_msg, 0, None).await
        };

        assert!(matches!(result.return_value(), Err(PSP22Error::Custom(_))));

        let balance_of_bob = balance_of!(client, address, Bob);

        assert!(matches!(balance_of_bob, 10));

        Ok(())
    }
}
