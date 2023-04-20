use anchor_lang::{
    prelude::*,
    solana_program::pubkey::Pubkey,
    solana_program::system_instruction
};
use crate::state::accounts::*;

// Function to renew a subscription
pub fn renew(
    ctx: Context<Renew>
) -> Result<()> {
    // Declare mutable references to the enterprise and user accounts
    let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
    let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
    // Transfer funds from the user's account to the enterprise's account
    // This is done using the Solana System Program's transfer instruction
    anchor_lang::solana_program::program::invoke(
        &system_instruction::transfer(&ctx.accounts.from.key(), &enterprise_data.authority, enterprise_data.amount_per_month),
        &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
    ).expect("Error");
    // Increment the total number of subscribed users for the enterprise
    enterprise_data.total_users += 1;
    // Set the user's renewal timestamp to one month from the current timestamp
    user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
    // Add 8 credits to the user's account
    user_data.credits += 8;
    // Return Ok(()) to indicate that the transaction was successful
    Ok(())
}

// Define the accounts required for the Renew function using the #[derive(Accounts)] macro
#[derive(Accounts)]
pub struct Renew<'info> {
    // The enterprise account that is being subscribed to
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    // The user account that is subscribing
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    // The account from which funds are transferred
    #[account(mut)]
    pub from: AccountInfo<'info>,
    // The account where stake is added to
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    // The signer for the user account
    pub user: Signer<'info>,
    // The Solana System Program
    pub system_program: Program<'info, System>,
}
