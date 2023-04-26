use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};
use crate::state::accounts::*;

// Define a function to delete a user's subscription data
pub fn delete(
    _ctx: Context<Delete> // The function takes a context object of type Delete
) -> Result<()> {
    Ok(())
}

// Define a struct to represent the accounts needed for the delete operation
#[derive(Accounts)]
pub struct Delete<'info> {
    // The enterprise_data account must be mutable and is derived from a seed
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    // The user_data account must be mutable, is derived from two seeds, and will be closed upon deletion
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump, close = enterprise_data)]
    pub user_data: Account<'info, SubscriberData>,
    // The user account must be a signer (i.e. it must be authorized to perform the delete operation)
    pub user: Signer<'info>,
    // The system_program account is a standard Solana program account used for system-level operations
    pub system_program: Program<'info, System>,
}
