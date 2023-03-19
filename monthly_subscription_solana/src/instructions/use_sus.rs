use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
}; 
use crate::state::accounts::*;
use crate::errors::ErrorCode;

pub fn use_sus(
    ctx: Context<UseSus>
) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    let mut secure_check: u8 = 0;
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp && user_data.credits > 0 { user_data.credits -= 1; secure_check += 1; }
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp { return Err(ErrorCode::OverdueCredits.into()); }
    if user_data.credits == 0 { 
        enterprise_data.total_users -= 1;
        return Err(ErrorCode::YouHaveNoCredits.into()); 
    }
    if secure_check == 0 { user_data.credits -= 1; } 
    Ok(())
}
#[derive(Accounts)]
pub struct UseSus<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}