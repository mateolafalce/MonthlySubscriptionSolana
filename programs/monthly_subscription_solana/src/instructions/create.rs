use crate::state::accounts::*;
use anchor_lang::{prelude::*, solana_program::pubkey::Pubkey};

pub fn create(ctx: Context<Create>, share_amount: u64, name: String) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let (_pda, bump) = Pubkey::find_program_address(
        &[b"Enterprise", ctx.accounts.user.key().as_ref()],
        ctx.program_id,
    );

    //update state
    enterprise_data.set_authority(ctx.accounts.user.key());
    enterprise_data.set_bump(bump);
    enterprise_data.set_name(name);
    enterprise_data.set_total_users();
    enterprise_data.set_amount_per_month(share_amount);
    enterprise_data.set_secure_check();

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
