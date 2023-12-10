use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Program,
};
use anyhow::Result;
use monthly_subscription_solana;

pub fn create(program: &Program, share_amount: u64, name: String) -> Result<()> {
    let enterprise_data: Keypair = Keypair::new();
    let tx: Signature = program
        .request()
        .accounts(monthly_subscription_solana::accounts::Create {
            enterprise_data: enterprise_data.pubkey(),
            user: program.payer(),
            system_program: system_program::ID,
        })
        .signer(&enterprise_data)
        .args(monthly_subscription_solana::instruction::Create { share_amount, name })
        .send()?;
    println!("Tx: {}", tx);
    Ok(())
}
