use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;

pub fn create(
    ctx: Context<Create>,
    share_amount: u64,
    name: String
) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let (_pda, bump) = Pubkey::find_program_address(&[b"Enterprise", ctx.accounts.user.key().as_ref()], ctx.program_id);
    enterprise_data.authority = ctx.accounts.user.key();
    enterprise_data.bump_original = bump;
    enterprise_data.name = name;
    enterprise_data.total_users = 0;
    enterprise_data.amount_per_month = share_amount;
    enterprise_data.secure_check = Clock::get().unwrap().unix_timestamp + 2332800;
    Ok(())
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"Enterprise", user.key().as_ref()], bump, payer = user, space = 8 + EnterpriseData::LEN)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}