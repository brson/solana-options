#![allow(unused)]

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



    /// If the token already exists then this instruction does nothing and
    /// returns the existing token's account.
    pub fn create_contract_token(ctx: Context<CreateContractToken>, desc: ContractTokenDesc) -> Result<Pubkey, ProgramError> {
        panic!()
    }

    pub fn get_contract_token(ctx: Context<GetContractToken>, desc: ContractTokenDesc) -> Result<Option<Pubkey>, ProgramError> {
        panic!()
    }

    /// Mint new options tokens by depositing collateral.
    ///
    /// Deposit collateral:
    ///
    /// for call contracts:
    ///
    /// - `qty` units of the underling (BTC in BTC/USD)
    ///
    /// for put contracts:
    ///
    /// - `strike_price` units of the base (USD in BTC/USD)
    ///
    /// User receives `qty` units of the contract token.
    pub fn mint_contract_tokens(ctx: Context<MintContractTokens>,
                                contract_token: Pubkey,
                                qty: u64) -> ProgramResult {
        panic!();
    }

    /// Called by holders of tokens to resolve the outcome of expiration.
    pub fn redeem_contract_tokens(ctx: Context<RedeemContractTokens>,
                                  contract_token: Pubkey) -> ProgramResult {
        panic!()
    }
}

#[account]
pub struct BrokerAccount {
    dummy: u32,
}

#[derive(Accounts)]
pub struct CreateContractToken<'info> {
    #[account(mut)]
    pub broker_account: ProgramAccount<'info, BrokerAccount>,
    pub payer: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct GetContractToken<'info> {
    #[account(mut)]
    pub broker_account: ProgramAccount<'info, BrokerAccount>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct ContractTokenDesc {
    pub underlying_serum_market: Pubkey,
    pub expiration_slot: u64, /* fixme should be Slot but doesn't work with anchor */
    pub strike_price: u64,
    pub kind: ContractKind,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ContractKind { Call, Put }

#[derive(Accounts)]
pub struct MintContractTokens<'info> {
    #[account(mut)]
    pub broker_account: ProgramAccount<'info, BrokerAccount>,
    pub user_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct RedeemContractTokens<'info> {
    #[account(mut)]
    pub broker_account: ProgramAccount<'info, BrokerAccount>,
    pub user_account: AccountInfo<'info>,
}






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
