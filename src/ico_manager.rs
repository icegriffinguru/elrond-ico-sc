#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

const EGLD_NUM_DECIMALS: usize = 18;

/// Manage ICO of a new ESDT
#[elrond_wasm::contract]
pub trait IcoManager {
    #[init]
    fn init(&self) {}

    /// endpoint ///
    
    #[payable("EGLD")]
    #[endpoint(buyTokens)]
    fn buy_tokens(&self, #[payment_amount] paid_amount: BigUint){
        self.require_activation();
        require!(paid_amount != 0, "you sent 0 EGLD");

        if !self.buy_limit().is_empty() {
            require!(paid_amount <= self.buy_limit().get(), "buy limit exceeded");
        }

        let caller = self.blockchain().get_caller();
        let dist_token_id = self.distributable_token_id().get();
        let price_per_token = self.distributable_token_price().get();
        let available_token_amount = self.blockchain().get_sc_balance(&dist_token_id, 0);

        let token_amount = &paid_amount / &price_per_token;

        require!(token_amount <= available_token_amount, "not enough tokens available");

        self.send().direct(&caller, &dist_token_id, 0, &token_amount, &[]);

        Ok(())
    }

    /// private functions ///
    
    fn require_activation(&self) {
        let starting_timestamp = self.activation_timestamp().get();
        let duration_timestamp = self.duration_timestamp().get();
        let current_timestamp = self.blockchain().get_block_timestamp();

        require!(current_timestamp >= starting_timestamp, "ICO is not started.");
        require!(current_timestamp < starting_timestamp + duration_timestamp, "ICO is finished.");
    }

    /// storage ///

    #[view(getTokenId)]
    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTokenAvailable)]
    #[storage_mapper("token_available")]
    fn token_available(&self) -> SingleValueMapper<BigUint>;

    // 1 ESDT price in EGLD-wei
    #[view(getTokenPrice)]
    #[storage_mapper("token_price")]
    fn token_price(&self) -> SingleValueMapper<BigUint>;

    #[view(getActivationTimestamp)]
    #[storage_mapper("activation_timestamp")]
    fn activation_timestamp(&self) -> SingleValueMapper<u64>;

    #[view(getDurationTimestamp)]
    #[storage_mapper("duration_timestamp")]
    fn duration_timestamp(&self) -> SingleValueMapper<u64>;
}
