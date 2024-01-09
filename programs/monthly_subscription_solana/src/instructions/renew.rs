use crate::{state::accounts::*, utils::util::ENTERPRISE};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn renew(_ctx: Context<Renew>) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct Renew<'info> {
    #[account(mut, seeds = [ENTERPRISE, enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    /// CHECK: secure
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: secure
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
