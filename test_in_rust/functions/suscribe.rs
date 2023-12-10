use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{
        pubkey::Pubkey,
        signature::{Keypair, Signature},
        signer::Signer,
    },
    Program,
};
use anyhow::Result;
use monthly_subscription_solana::state::EnterpriseData;

pub fn suscribe(
    program: &Program,
    name: String,
    lastname: String,
    enterprise_data: Pubkey,
) -> Result<()> {
    let user_data: Keypair = Keypair::new();
    let account: EnterpriseData = program.account(enterprise_data)?;
    let tx: Signature = program
        .request()
        .accounts(monthly_subscription_solana::accounts::Suscribe {
            enterprise_data,
            user_data: user_data.pubkey(),
            from: program.payer(),
            stake: account.authority,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .signer(&user_data)
        .args(monthly_subscription_solana::instruction::Suscribe { name, lastname })
        .send()?;
    println!("Tx: {}", tx);
    Ok(())
}
