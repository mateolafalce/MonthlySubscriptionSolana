use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    create::create, delete::delete, renew::renew, suscribe::suscribe, use_sus::use_sus,
};

declare_id!("4tGecsChPUDT1P1hN4MP3AuM83jugP1Jd2w9VFwmTbev");

#[program]
pub mod monthly_subscription_solana {
    use super::*;

    pub fn create_(ctx: Context<Create>, share_amount: u64, name: String) -> Result<()> {
        create(ctx, share_amount, name)
    }

    pub fn use_sus_(ctx: Context<UseSus>) -> Result<()> {
        use_sus(ctx)
    }

    pub fn suscribe_(ctx: Context<Suscribe>, name: String, lastname: String) -> Result<()> {
        suscribe(ctx, name, lastname)
    }

    pub fn renew_(ctx: Context<Renew>) -> Result<()> {
        renew(ctx)
    }

    pub fn delete_(ctx: Context<Delete>) -> Result<()> {
        delete(ctx)
    }
}
