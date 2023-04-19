use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
};

// Import the account and error types from the `state` and `errors` modules
use crate::state::accounts::*;
use crate::errors::ErrorCode;

// Define the `use_sus` function, which is used to decrement a user's credits when they use the service
pub fn use_sus(
    ctx: Context<UseSus> // The function takes a `Context` object as a parameter
) -> Result<()> {
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data; // Get a mutable reference to the enterprise data account
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data; // Get a mutable reference to the user data account
    let mut secure_check: u8 = 0; // Initialize a variable to keep track of whether a secure check has been performed
    // If the user's credits are greater than zero and their subscription has not expired, decrement their credits and perform a secure check
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp && user_data.credits > 0 {
        user_data.credits -= 1;
        secure_check += 1;
    }
    // If the user's subscription has expired, return an error
    if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp {
        return Err(ErrorCode::OverdueCredits.into());
    }
    // If the user has no credits left, decrement the total number of users and return an error
    if user_data.credits == 0 {
        enterprise_data.total_users -= 1;
        return Err(ErrorCode::YouHaveNoCredits.into());
    }
    // If a secure check has not been performed, decrement the user's credits and return Ok(())
    if secure_check == 0 {
        user_data.credits -= 1;
    }
    Ok(()) // Return Ok(()) if the function completes successfully
}

// Define a `UseSus` struct that represents the accounts needed to execute the `use_sus` function
#[derive(Accounts)]
pub struct UseSus<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>, // The enterprise data account, which is mutable and requires a seed
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>, // The user data account, which is mutable and requires two seeds
    pub user: Signer<'info>, // The user's signature, which is required to execute the function
    pub system_program: Program<'info, System>, // The system program account, which is required to execute the function
}
