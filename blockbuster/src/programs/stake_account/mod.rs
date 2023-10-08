use crate::{
    error::BlockbusterError,
    programs::{ProgramParseResult, stake_account::state::SacStakeAccount}, program_handler::{ParseResult, ProgramParser}
};
use borsh::BorshDeserialize;
use plerkle_serialization::AccountInfo;
use solana_sdk::{pubkey::Pubkey, pubkeys, borsh::try_from_slice_unchecked};

pub mod state;

pubkeys!(
    stake_program_id,
    "3gHZaQrR1pDfNHodJydGZ3MCnMVD3BtEd9uNAAnDY2vr"
);

pub enum StakeProgramAccount {
    SacStakeAccount(SacStakeAccount),
    EmptyAccount
}

impl ParseResult for StakeProgramAccount {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::StakeProgramAccount(self)
    }
}

pub struct StakeAccountParser;

impl ProgramParser for StakeAccountParser {
    fn key(&self) -> Pubkey {
        stake_program_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &stake_program_id()
    }
    fn handles_account_updates(&self) -> bool {
        true
    }

    fn handles_instructions(&self) -> bool {
        false
    }
    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<dyn ParseResult + 'static>, BlockbusterError> {
        let account_data = if let Some(account_info) = account_info.data() {
            account_info.iter().collect::<Vec<_>>()
        } else {
            return Err(BlockbusterError::DeserializationError);
        };

        let account_type = match account_data.len() {
            300 => {
                let stake_account = SacStakeAccount::deserialize(&mut &account_data[..])?;
                StakeProgramAccount::SacStakeAccount(stake_account)
            }
            _ => {
                return Err(BlockbusterError::InvalidDataLength);
            }
        };

        Ok(Box::new(account_type))
    }
}
