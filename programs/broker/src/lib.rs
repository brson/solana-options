use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Slot;

#[program]
mod broker {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> ProgramResult {
        let my_account = &mut ctx.accounts.my_account;
        my_account.data = data;
        Ok(())
    }




    /// If the token already exists then this instruction does nothing.
    pub fn create_contract_token(ctx: Context<CreateContractToken>, data: CreateContractTokenData) -> ProgramResult {
        #![allow(unused)]
        panic!()
    }
}

#[account]
pub struct BrokerAccount {
}

#[derive(Accounts)]
pub struct CreateContractToken<'info> {
    #[account(mut)]
    pub broker_account: ProgramAccount<'info, BrokerAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateContractTokenData {
    underlying_serum_market: Pubkey,
    expiration_slot: Slot,
    strike_price: u64,
    kind: ContractKind,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ContractKind { Call, Put }



#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(zero)]
    pub my_account: ProgramAccount<'info, MyAccount>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub my_account: ProgramAccount<'info, MyAccount>,
}

#[account]
pub struct MyAccount {
    pub data: u64,
}
