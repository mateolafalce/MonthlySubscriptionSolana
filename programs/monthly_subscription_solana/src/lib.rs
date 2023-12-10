use anchor_lang::prelude::*;
use instructions::*;

pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

declare_id!("4tGecsChPUDT1P1hN4MP3AuM83jugP1Jd2w9VFwmTbev");

#[program]
pub mod monthly_subscription_solana {
    use super::*;
    pub fn create(ctx: Context<Create>, share_amount: u64, name: String) -> Result<()> {
        instructions::create::create(ctx, share_amount, name)
    }
    pub fn use_sus(ctx: Context<UseSus>) -> Result<()> {
        instructions::use_sus::use_sus(ctx)
    }
    pub fn suscribe(ctx: Context<Suscribe>, name: String, lastname: String) -> Result<()> {
        instructions::suscribe::suscribe(ctx, name, lastname)
    }
    pub fn renew(ctx: Context<Renew>) -> Result<()> {
        instructions::renew::renew(ctx)
    }
    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        instructions::delete::delete(ctx)
    }
}
