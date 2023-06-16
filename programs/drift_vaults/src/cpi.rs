use anchor_lang::prelude::*;

pub trait InitializeUserCPI {
    fn drift_initialize_user(&self, name: [u8; 32], bump: u8) -> Result<()>;

    fn drift_initialize_user_stats(&self, name: [u8; 32], bump: u8) -> Result<()>;
}

pub trait DepositCPI {
    fn drift_deposit(&self, amount: u64) -> Result<()>;
}

pub trait WithdrawCPI {
    fn drift_withdraw(&self, amount: u64) -> Result<()>;
}

pub trait UpdateUserCPI {
    fn drift_update_user_delegate(&self, delegate: Pubkey) -> Result<()>;
}

pub trait TokenTransferCPI {
    fn token_transfer(&self, amount: u64) -> Result<()>;
}
