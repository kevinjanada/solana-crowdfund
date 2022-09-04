use solana_program::{
    pubkey::Pubkey,
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult, program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar}, program_pack::{Pack, IsInitialized}, system_instruction, msg,
    program::invoke_signed,
};

use crate::{
    instruction::{CreateFundPayload, CrowdfundInstruction},
    state::Crowdfund
};

pub struct Processor;

impl Processor {
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
        let instruction = CrowdfundInstruction::unpack(instruction_data)?;
        match instruction {
            CrowdfundInstruction::CreateFund { payload } => {
                Self::process_create_fund(program_id, accounts, payload)
            },
        }
    }

    fn process_create_fund(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        payload: CreateFundPayload,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();

        let initializer = next_account_info(account_info_iter)?;
        let crowdfund_account = next_account_info(account_info_iter)?;
        let system_program = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let (pda, bump) = Pubkey::find_program_address(
            &[b"crowdfund".as_ref(), initializer.key.as_ref()],
            program_id,
        );
        if pda != *crowdfund_account.key {
            return Err(ProgramError::InvalidAccountData);
        }

        let rent = Rent::get()?;
        let rent_lamports = rent.minimum_balance(Crowdfund::LEN);

        // Create crowdfund_account using systemprogram
        // Set the program as the owner
        let create_crowdfund_account_ix = &system_instruction::create_account(
            initializer.key,
            crowdfund_account.key,
            rent_lamports,
            Crowdfund::LEN.try_into().unwrap(),
            program_id
        );
        msg!("Creating Crowdfund account");
        invoke_signed(
            create_crowdfund_account_ix,
            &[
                initializer.clone(),
                crowdfund_account.clone(),
                system_program.clone(),
            ], 
            &[&[
                b"crowdfund".as_ref(),
                initializer.key.as_ref(),
                &[bump]
            ]]
        )?;

        let mut crowdfund_info = Crowdfund::unpack_unchecked(
            &crowdfund_account.try_borrow_data()?
        )?;
        if crowdfund_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let CreateFundPayload { name, goal_amount, deadline } = payload;
        crowdfund_info.is_initialized = true;
        crowdfund_info.name = name;
        crowdfund_info.goal_amount = goal_amount;
        crowdfund_info.deadline = deadline;
        crowdfund_info.initializer_pubkey = *initializer.key;

        Crowdfund::pack(
            crowdfund_info,
            &mut crowdfund_account.try_borrow_mut_data()?
        )?;

        Ok(())
    }
}
