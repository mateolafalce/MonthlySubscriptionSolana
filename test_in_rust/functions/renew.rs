use anchor_client::{
    anchor_lang::system_program,
    solana_sdk::{pubkey::Pubkey, signature::Signature},
    Program,
};
use anyhow::Result;
use monthly_subscription_solana;
use monthly_subscription_solana::state::EnterpriseData;

pub fn renew(program: &Program, enterprise_data: Pubkey, user_data: Pubkey) -> Result<()> {
    let account: EnterpriseData = program.account(enterprise_data)?;
    let tx: Signature = program
        .request()
        .accounts(monthly_subscription_solana::accounts::Renew {
            enterprise_data,
            user_data,
            from: program.payer(),
            stake: account.authority,
            user: program.payer(),
            system_program: system_program::ID,
        })
        .args(monthly_subscription_solana::instruction::Renew {})
        .send()?;
    println!("Tx: {}", tx);
    Ok(())
}
