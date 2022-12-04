use crate::*;

impl DepositProxy {
    pub(crate) fn internal_new(depository_account: AccountId) -> Self {
        assert!(
            !env::state_exists(),
            "Contract state is already initialize."
        );

        assert!(
            env::is_valid_account_id(depository_account.as_bytes()),
            "The deposit account ID is invalid"
        );

        Self { depository_account }
    }

    pub(crate) fn internal_deposit(&self) -> Promise {
        let account = env::signer_account_id();
        let amount = env::attached_deposit();

        let message = format!("@{} trying to deposit {}.", account, amount);
        log!(message);

        let promise = depository::ext(self.depository_account.clone())
            .with_attached_deposit(amount)
            .with_static_gas(Gas(5 * TGAS))
            .deposit();

        promise.then(
            Self::ext(env::current_account_id())
                .with_static_gas(Gas(5 * TGAS))
                .query_deposit_callback(),
        )
    }
}
