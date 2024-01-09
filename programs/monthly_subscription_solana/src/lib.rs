use anchor_lang::prelude::*;
use instructions::*;

pub mod instructions;
pub mod state;
pub mod utils;

use instructions::{
    create::create, delete::delete, renew::renew, suscribe::suscribe, use_sus::use_sus,
};

declare_id!("FwNe8kWDNBidqmntkHx72XuHy6j2DJ4a62PibXAVU7vX");

#[program]
pub mod monthly_subscription_solana {
    use super::*;

    pub fn create_enterprise(ctx: Context<Create>, share_amount: u64, name: String) -> Result<()> {
        create(ctx, share_amount, name)
    }

    pub fn use_suscription(ctx: Context<UseSus>) -> Result<()> {
        use_sus(ctx)
    }

    pub fn suscribe_business(ctx: Context<Suscribe>, name: String, lastname: String) -> Result<()> {
        suscribe(ctx, name, lastname)
    }

    pub fn renew_suscription(ctx: Context<Renew>) -> Result<()> {
        renew(ctx)
    }

    pub fn delete_account(ctx: Context<Delete>) -> Result<()> {
        delete(ctx)
    }
}
