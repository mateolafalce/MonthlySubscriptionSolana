use crate::{
    state::accounts::*,
    utils::util::{ENTERPRISE, MAX_NAME},
};
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn create(ctx: Context<Create>, share_amount: u64, name: String) -> Result<()> {
    let user: Pubkey = ctx.accounts.user.key();
    let (pda, bump) = Pubkey::find_program_address(&[ENTERPRISE, user.as_ref()], ctx.program_id);
    require_keys_eq!(pda, ctx.accounts.enterprise_data.key());
    require_gte!(MAX_NAME, name.len());
    //update state
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    enterprise_data.set_authority(user);
    enterprise_data.set_bump(bump);
    enterprise_data.set_name(name);
    enterprise_data.set_total_users();
    enterprise_data.set_amount_per_month(share_amount);
    enterprise_data.set_secure_check();
    Ok(())
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [ENTERPRISE, user.key().as_ref()], bump, payer = user, space = EnterpriseData::LEN)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
