use borsh::{BorshSerialize, BorshDeserialize};
use solana_sdk::{pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct SacStakeAccount {
    pub descriminator: u64,
    pub authority: Pubkey,
    pub token: Pubkey,
    pub start_staking: i64,
    pub last_withdraw: i64,
    pub bump: u8,
    pub vault_token_account_bump: u8,
}
