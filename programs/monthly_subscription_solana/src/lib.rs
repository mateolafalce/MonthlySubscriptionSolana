use anchor_lang::{
    prelude::*,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;
declare_id!("8SMdQGKPcJsMBGQjR79XRBPVec2Z8BJi89wFfRqZqTFK");

#[program]
pub mod monthly_subscription_solana {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        amount: u64
    ) -> Result<()> {
        let suscribe_data: &mut Account<SubscribeData> = &mut ctx.accounts.suscribe_data;
        let (pda, bump) = Pubkey::find_program_address(&[b"MyEnterprise", ctx.accounts.user.key().as_ref()], &Pubkey::from_str("8SMdQGKPcJsMBGQjR79XRBPVec2Z8BJi89wFfRqZqTFK").unwrap());
        suscribe_data.authority = ctx.accounts.user.key();
        suscribe_data.pda = pda;
        suscribe_data.bump_original = bump;
        suscribe_data.amount_per_month = amount;
        msg!("PDA: {}", suscribe_data.pda);
        msg!("Amount per month: {}", suscribe_data.amount_per_month);  //CUSTOMIZABLE
        Ok(())
    }
    pub fn suscribe(
        ctx: Context<Suscribe>
    ) -> Result<()> {
        let suscribe_data: &mut Account<SubscribeData> = &mut ctx.accounts.suscribe_data;
        let suscribe_to: &mut Account<SubscribeState> = &mut ctx.accounts.suscribe_to;
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &suscribe_data.pda, suscribe_data.amount_per_month),
            &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
        ).expect("Error");
        suscribe_to.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"MyEnterprise", user.key().as_ref()], bump, payer = user, space = 81)]
    pub suscribe_data: Account<'info, SubscribeData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Suscribe<'info> {
    #[account(mut, seeds = [b"MyEnterprise", suscribe_data.authority.key().as_ref()], bump = suscribe_data.bump_original)]
    pub suscribe_data: Account<'info, SubscribeData>,
    #[account(init, seeds = [b"MyEnterprise-Suscribe", user.key().as_ref()], bump, payer = from, space = 16)]
    pub suscribe_to: Account<'info, SubscribeState>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct SubscribeData {
    pub authority: Pubkey, 
    pub pda: Pubkey,
    pub bump_original: u8, 
    pub amount_per_month: u64
}
#[account]
pub struct SubscribeState {
    pub month_timestamp: i64
}