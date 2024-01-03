use anchor_client::{
    solana_sdk::{pubkey::Pubkey, signature::read_keypair_file},
    Client, Cluster,
};
use anyhow::Result;
use std::{rc::Rc, str::FromStr};

pub mod functions;

pub use functions::{create::create, suscribe::suscribe, use_sus::use_sus, renew::renew, delete::delete};

fn main() -> Result<()> {
    let pubkey: Pubkey = Pubkey::from_str("6pms4FybaDt64uxEt4qvnWE5tXm4256mmnUe2b61sqj3").unwrap();
    let org_name: String = "Gym Go".to_string();
    let client: Client = Client::new(
        Cluster::Devnet,
        Rc::new(
            read_keypair_file(&*shellexpand::tilde(
                "/.json",
            ))
            .expect("Example requires a keypair file"),
        ),
    )
    .program(pubkey);
    create(&client, 2545, org_name)?;
    suscribe(
        &program,
        "Jhon".to_string(),
        "Chigurth".to_string(),
        pubkey,
    )?;
    use_sus(&client, pubkey, pubkey)?;
    renew(&client, pubkey, pubkey)?;
    delete(&client, pubkey, pubkey)?;
    Ok(())
}
