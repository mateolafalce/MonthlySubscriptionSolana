use crate::{
    state::accounts::*,
    utils::util::{ENTERPRISE, MAX_NAME},
};
use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, pubkey::Pubkey, system_instruction::transfer},
};

pub fn suscribe(ctx: Context<Suscribe>, name: String, lastname: String) -> Result<()> {
    let from: Pubkey = ctx.accounts.from.key();
    let to: Pubkey = ctx.accounts.enterprise_data.authority;
    let amount: u64 = ctx.accounts.enterprise_data.amount_per_month;
    //validations
    check_size(name, MAX_NAME).unwrap();
    check_size(lastname, MAX_NAME).unwrap();
    let (pda, bump) = Pubkey::find_program_address(
        &[
            ctx.accounts.enterprise_data.key().as_ref(),
            ctx.accounts.user.key().as_ref(),
        ],
        ctx.program_id,
    );
    require_keys_eq!(pda, ctx.accounts.user_data.key());
    //transfer from -> to (lamport)
    invoke(
        &transfer(&from, &to, amount),
        &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.stake.to_account_info().clone(),
        ],
    )
    .expect("Error");
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    //update state
    enterprise_data.add_total_users();
    user_data.set_bump(bump);
    user_data.add_month_timestamp();
    user_data.add_credits();
    Ok(())
}

pub fn check_size(string: String, size: usize) -> Result<()> {
    require_gt!(size, string.chars().count());
    Ok(())
}

#[derive(Accounts)]
pub struct Suscribe<'info> {
    #[account(mut, seeds = [ENTERPRISE, enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(init, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump, payer = from, space = SubscriberData::LEN)]
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
