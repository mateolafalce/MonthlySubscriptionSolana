use crate::{state::accounts::*, utils::util::ENTERPRISE};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn delete(_ctx: Context<Delete>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, seeds = [ENTERPRISE, enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump, close = enterprise_data)]
    pub user_data: Account<'info, SubscriberData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
