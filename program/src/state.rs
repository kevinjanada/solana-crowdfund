use arrayref::{array_mut_ref, mut_array_refs};
use solana_program::{program_error::ProgramError, msg};

use {
    solana_program::{
        pubkey::Pubkey,
        clock::UnixTimestamp,
        program_pack::{IsInitialized, Pack, Sealed},
    },
    arrayref::{
        array_ref,
        array_refs,
    },
};


pub struct Crowdfund {
    pub is_initialized: bool,
    pub name: String,
    pub initializer_pubkey: Pubkey,
    pub goal_amount: u64,
    pub deadline: UnixTimestamp,
    pub bump: u8,
}

impl Sealed for Crowdfund {}

impl IsInitialized for Crowdfund {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

pub const INITIALIZED_SIZE: usize = 1;
pub const NAME_LENGTH_SIZE: usize = 4; // 4 bytes stores the length of string 
pub const NAME_SIZE: usize = 256;
pub const INITIALIZER_PUBKEY_SIZE: usize = 32; 
pub const GOAL_AMOUNT_SIZE: usize = 8;
pub const DEADLINE_SIZE: usize = 8;
pub const BUMP_SIZE: usize = 1;

pub const CROWDFUND_ACCOUNT_SIZE: usize =
    INITIALIZED_SIZE +
    NAME_LENGTH_SIZE +
    NAME_SIZE +
    INITIALIZER_PUBKEY_SIZE +
    GOAL_AMOUNT_SIZE + 
    DEADLINE_SIZE +
    BUMP_SIZE;

impl Pack for Crowdfund {
    const LEN: usize = CROWDFUND_ACCOUNT_SIZE;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, solana_program::program_error::ProgramError> {
        let src = array_ref![src, 0, Crowdfund::LEN];
        let (
            is_initialized,
            _name_length,
            name,
            initializer_pubkey,
            goal_amount,
            deadline,
            bump,
        ) = array_refs![
            src,
            INITIALIZED_SIZE,
            NAME_LENGTH_SIZE,
            NAME_SIZE,
            INITIALIZER_PUBKEY_SIZE,
            GOAL_AMOUNT_SIZE,
            DEADLINE_SIZE,
            BUMP_SIZE
        ];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData)
        };

        Ok(Crowdfund {
            is_initialized,
            name: String::from_utf8_lossy(name).to_string(),
            initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
            goal_amount: u64::from_le_bytes(*goal_amount),
            deadline: UnixTimestamp::from_le_bytes(*deadline),
            bump: u8::from_le_bytes(*bump),
        })
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let Crowdfund {
            is_initialized,
            name,
            initializer_pubkey,
            goal_amount,
            deadline,
            bump
        } = self;

        let dst = array_mut_ref![dst, 0, Crowdfund::LEN];
        let (
            is_initialized_dst,
            name_length_dst,
            name_dst,
            initializer_pubkey_dst,
            goal_amount_dst,
            deadline_dst,
            bump_dst,
        ) = mut_array_refs![
            dst,
            INITIALIZED_SIZE,
            NAME_LENGTH_SIZE,
            NAME_SIZE,
            INITIALIZER_PUBKEY_SIZE,
            GOAL_AMOUNT_SIZE,
            DEADLINE_SIZE,
            BUMP_SIZE
        ];

        is_initialized_dst[0] = *is_initialized as u8;
        name_length_dst[..].copy_from_slice(&(name.len() as u32).to_le_bytes());
        name_dst[..name.len()].clone_from_slice(name.as_bytes());
        initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
        goal_amount_dst.copy_from_slice(&goal_amount.to_le_bytes());
        deadline_dst.copy_from_slice(&deadline.to_le_bytes());
        bump_dst.copy_from_slice(&bump.to_le_bytes());
    }
}
