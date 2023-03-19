use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
    solana_program::system_instruction
};
use crate::state::accounts::*;

pub fn renew(
    ctx: Context<Renew>
) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &enterprise_data.authority, enterprise_data.amount_per_month),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
    ).expect("Error");
    enterprise_data.total_users += 1;
    user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
    user_data.credits += 8;
    Ok(())
}
#[derive(Accounts)]
pub struct Renew<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
