use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::Signature,
        pubkey::Pubkey,
    },
    Program,
};
use anyhow::Result;
use monthly_subscription_solana;

pub fn use_sus(
    program: &Program,
    enterprise_data: Pubkey,
    user_data: Pubkey
) -> Result<()> {
    let tx: Signature = program
        .request()
        .accounts(monthly_subscription_solana::accounts::UseSus {
            enterprise_data,
            user_data,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(monthly_subscription_solana::instruction::UseSus {})
        .send()?;
    println!("Tx: {}", tx);
    Ok(())
}