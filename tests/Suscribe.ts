import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MonthlySubscriptionSolana } from "../target/types/monthly_subscription_solana";
import { Enterprise } from "/mnt/c/Users/Mateo/monthly_subscription_Solana./monthly_subscription_Solana./tests/Accounts";
import { PublicKey } from '@solana/web3.js';
const { SystemProgram } = anchor.web3;

describe("Subscribe", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const wallet = provider.wallet as anchor.Wallet;
    const program = anchor.workspace.MonthlySubscriptionSolana as Program<MonthlySubscriptionSolana>;
  it('Creating subscription', async () => {
    const Account = await program.account.enterpriseData.fetch(Enterprise)
    const [userData, _bump] = await PublicKey.findProgramAddress(
      [
        Enterprise.toBuffer(),
        wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const tx = await program.methods.suscribe(
      "Mateo",
      "Lafalce"
    ).accounts({
        enterpriseData: Enterprise,
        userData: userData,
        from: wallet.publicKey,
        stake: Account.authority,
        user: wallet.publicKey,
        systemProgram: SystemProgram.programId
      }).rpc();
      console.log("---------------------------------------------")
      console.log("PDA: ", userData.toBase58())
      console.log("---------------------------------------------")
      console.log("Tx: ", tx);
      console.log("---------------------------------------------")
  });
});