// Import necessary modules
use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;

// Define the create function with Context and necessary parameters
pub fn create(
    ctx: Context<Create>,
    share_amount: u64,
    name: String
) -> Result<()> {
    // Get the enterprise_data account and PDA (program-derived address)
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let (_pda, bump) = Pubkey::find_program_address(&[b"Enterprise", ctx.accounts.user.key().as_ref()], ctx.program_id);
    // Set the authority, name, and other fields in the enterprise_data account
    enterprise_data.authority = ctx.accounts.user.key();
    enterprise_data.bump_original = bump;
    enterprise_data.name = name;
    enterprise_data.total_users = 0;
    enterprise_data.amount_per_month = share_amount;
    enterprise_data.secure_check = Clock::get().unwrap().unix_timestamp + 2332800;
    // Return Ok if successful
    Ok(())
}

// Define a struct for the create function's accounts
#[derive(Accounts)]
pub struct Create<'info> {
    // Initialize enterprise_data account with seeds, payer, and space
    #[account(init, seeds = [b"Enterprise", user.key().as_ref()], bump, payer = user, space = 8 + EnterpriseData::LEN)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut)]
    pub user: Signer<'info>, // mutable user account
    pub system_program: Program<'info, System>, // system program account
}
