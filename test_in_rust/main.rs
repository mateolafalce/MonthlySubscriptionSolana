use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
    Client, Cluster,
};
use anyhow::Result;
use std::rc::Rc;
use std::str::FromStr;
pub mod functions;

fn main() -> Result<()> {
    let program = Client::new(
        Cluster::Devnet,
        Rc::new(
            read_keypair_file(&*shellexpand::tilde(
                "C:/Users/Mateo/.config/solana/id.json",
            ))
            .expect("Example requires a keypair file"),
        ),
    )
    .program(Pubkey::from_str("6pms4FybaDt64uxEt4qvnWE5tXm4256mmnUe2b61sqj3").unwrap());
    let enterprise_data: Pubkey =
        Pubkey::from_str("6pms4FybaDt64uxEt4qvnWE5tXm4256mmnUe2b61sqj3").unwrap();
    let user_data: Pubkey =
        Pubkey::from_str("6pms4FybaDt64uxEt4qvnWE5tXm4256mmnUe2b61sqj3").unwrap();
    functions::create::create(&program, 2545, "Gym Go".to_string())?;
    functions::suscribe::suscribe(
        &program,
        "Jhon".to_string(),
        "Chigurth".to_string(),
        enterprise_data,
    )?;
    functions::use_sus::use_sus(&program, enterprise_data, user_data)?;
    functions::renew::renew(&program, enterprise_data, user_data)?;
    functions::delete::delete(&program, enterprise_data, user_data)?;
    Ok(())
}
