// Import necessary libraries
use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
    solana_program::system_instruction,
};
// Import custom modules
use crate::state::accounts::*;
use crate::errors::ErrorCode;

// Define function to subscribe
pub fn suscribe(
    ctx: Context<Suscribe>,
    name: String,
    lastname: String
) -> Result<()> {
    // Check if name or lastname are too long
    if name.chars().count() > 20 {
        return Err(ErrorCode::TooLong.into())
    }
    if lastname.chars().count() > 20 {
        return Err(ErrorCode::TooLong.into())
    }
    // Get mutable reference to user_data account
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    // Generate PDA and bump using the keys of the enterprise and user accounts
    let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.enterprise_data.key().as_ref(), ctx.accounts.user.key().as_ref()], ctx.program_id);
    // Transfer funds from 'from' account to the enterprise authority account
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.enterprise_data.authority, ctx.accounts.enterprise_data.amount_per_month),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
    ).expect("Error");
    // Get mutable reference to enterprise_data account
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    // Increment total_users count
    enterprise_data.total_users += 1;
    // Set user_data bump, month_timestamp, and credits
    user_data.bump = bump;
    user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
    user_data.credits = 8;
    // Return Ok if everything is successful
    Ok(())
}

// Define accounts required for subscription
#[derive(Accounts)]
pub struct Suscribe<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(init, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump, payer = from, space = 8 + SubscriberData::LEN)]
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
