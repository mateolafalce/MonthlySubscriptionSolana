use anchor_lang::{
    prelude::*,
    solana_program::system_instruction,
    solana_program::pubkey::Pubkey,
}; 
use std::str::FromStr;
declare_id!("GySjZVLx2nSYBT6zhhpk5YsyqNhngwZa28AmXosK8bjv");

#[program]
pub mod monthly_subscription_solana {
    use super::*;

    pub fn create(
        ctx: Context<Create>,
        share_amount: u64,
        name: String
    ) -> Result<()> {
        let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
        let (_pda, bump) = Pubkey::find_program_address(&[b"Enterprise", ctx.accounts.user.key().as_ref()], &Pubkey::from_str("GySjZVLx2nSYBT6zhhpk5YsyqNhngwZa28AmXosK8bjv").unwrap());
        enterprise_data.authority = ctx.accounts.user.key();
        enterprise_data.bump_original = bump;
        enterprise_data.name = name;
        enterprise_data.total_users = 0;
        enterprise_data.amount_per_month = share_amount;
        enterprise_data.secure_check = Clock::get().unwrap().unix_timestamp + 2332800;
        Ok(())
    }
    pub fn suscribe(
        ctx: Context<SuscribeIt>,
        name: String,
        lastname: String
    ) -> Result<()> {
        if name.chars().count() > 20 {
            return Err(ErrorCode::TooLong.into())
        }
        if lastname.chars().count() > 20 {
            return Err(ErrorCode::TooLong.into())
        }
        let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
        let (_pda, bump) = Pubkey::find_program_address(&[ctx.accounts.enterprise_data.key().as_ref(), ctx.accounts.user.key().as_ref()], &Pubkey::from_str("GySjZVLx2nSYBT6zhhpk5YsyqNhngwZa28AmXosK8bjv").unwrap());
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &ctx.accounts.enterprise_data.authority, ctx.accounts.enterprise_data.amount_per_month),
            &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
        ).expect("Error");
        let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
        enterprise_data.total_users += 1;
        user_data.bump = bump;
        user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
        user_data.credits = 8;
        Ok(())
    }
    pub fn use_your_credits(
        ctx: Context<Use>
    ) -> Result<()> {
        let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
        let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
        let mut secure_check: u8 = 0;
        if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp && user_data.credits > 0 { user_data.credits -= 1; secure_check += 1; }
        if user_data.month_timestamp < Clock::get().unwrap().unix_timestamp { return Err(ErrorCode::OverdueCredits.into()); }
        if user_data.credits == 0 { 
            enterprise_data.total_users -= 1;
            return Err(ErrorCode::YouHaveNoCredits.into()); 
        }
        if secure_check == 0 { user_data.credits -= 1; } 
        Ok(())
    }
    pub fn renew_subscription(
        ctx: Context<Renew>
    ) -> Result<()> {
        let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
        let user_data: &mut Account<SubscriberData> = &mut ctx.accounts.user_data;
        anchor_lang::solana_program::program::invoke(
            &system_instruction::transfer(&ctx.accounts.from.key(), &enterprise_data.authority, enterprise_data.amount_per_month),
            &[ctx.accounts.from.to_account_info(), ctx.accounts.stake.to_account_info().clone()],
        ).expect("Error");
        enterprise_data.total_users += 1;
        user_data.month_timestamp = Clock::get().unwrap().unix_timestamp + 2592000;
        user_data.credits += 8;
        Ok(())
    }
    pub fn modify_share(
        ctx: Context<Create>,
        share_amount: u64
    ) -> Result<()> {
        let enterprise_data: &mut Account<EnterpriseData> = &mut ctx.accounts.enterprise_data;
        if enterprise_data.secure_check < Clock::get().unwrap().unix_timestamp {
            return Err(ErrorCode::RecentlyChanged.into());
        }
        enterprise_data.secure_check += 2332800;
        enterprise_data.amount_per_month = share_amount;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, seeds = [b"Enterprise", user.key().as_ref()], bump, payer = user, space = 93)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct SuscribeIt<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(init, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump, payer = from, space = 98)]
    pub user_data: Account<'info, SubscriberData>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Use<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
pub struct Renew<'info> {
    #[account(mut, seeds = [b"Enterprise", enterprise_data.authority.key().as_ref()], bump = enterprise_data.bump_original)]
    pub enterprise_data: Account<'info, EnterpriseData>,
    #[account(mut, seeds = [enterprise_data.key().as_ref(), user.key().as_ref()], bump = user_data.bump)]
    pub user_data: Account<'info, SubscriberData>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub from: AccountInfo<'info>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(mut)]
    pub stake: AccountInfo<'info>,
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct EnterpriseData {
    pub authority: Pubkey, 
    pub name: String, //30
    pub bump_original: u8, 
    pub amount_per_month: u64,
    pub secure_check: i64,
    pub total_users: u16
}
#[account]
pub struct SubscriberData {
    pub authority: Pubkey,
    pub name: String, // 20
    pub lastname: String, // 20
    pub month_timestamp: i64,
    pub bump: u8,
    pub credits: u8
}
#[error_code]
pub enum ErrorCode {
    #[msg("Overdue credits")]OverdueCredits, #[msg("You have no credits")]YouHaveNoCredits, 
    #[msg("Recently changed less than 22 days ago")]RecentlyChanged, #[msg("Only 20 characters")]TooLong, 
}