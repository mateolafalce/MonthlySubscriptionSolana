use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::Signature,
    },
    Program,
};
use anyhow::Result;
use monthly_subscription_solana;

pub fn delete(
    program: &Program,
    enterprise_data: Pubkey,
    user_data: Pubkey
) -> Result<()> {
    let tx: Signature = program
        .request()
        .accounts(monthly_subscription_solana::accounts::Delete {
            enterprise_data,
            user_data,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(monthly_subscription_solana::instruction::Delete {})
        .send()?;
    println!("Tx: {}", tx);
    Ok(())
}