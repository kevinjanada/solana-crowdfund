use {
    solana_program::clock::UnixTimestamp,
    solana_program::program_error::ProgramError,
    borsh::BorshDeserialize,
};

#[derive(Debug, PartialEq)]
pub enum CrowdfundInstruction {
    /// Create a new `Crowdfund`
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person creating the Crowdfund
    /// 1. `[writable]` The Crowdfund account that will hold information relevant to the funding campaign
    /// 2. `[]` The system program
    CreateFund {
        payload: CreateFundPayload,
    }
}

#[derive(BorshDeserialize, Debug, PartialEq)]
pub struct CreateFundPayload {
    pub name: String,
    pub goal_amount: u64,
    pub deadline: UnixTimestamp,
}

impl CrowdfundInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (ix_id, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        match ix_id {
            0 => {
                let payload = CreateFundPayload::try_from_slice(rest)?;
                Ok(Self::CreateFund {
                    payload
                })
            },
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
