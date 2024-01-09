use crate::{state::accounts::*, utils::util::ENTERPRISE};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn use_sus(ctx: Context<UseSus>) -> Result<()> {
    require_keys_eq!(
        ctx.accounts.user.key(),
        ctx.accounts.enterprise_data.authority.key()
    );
    //get &mut accounts
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    //validations
    enterprise_data.have_credits(user_data.credits).unwrap();
    user_data.valid_time().unwrap();
    //update state
    user_data.sub_credits();
    Ok(())
}

#[derive(Accounts)]
pub struct UseSus<'info> {
    #[account(mut, seeds = [ENTERPRISE, enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
