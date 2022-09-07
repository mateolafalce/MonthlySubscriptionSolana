import * as anchor from "@project-serum/anchor";
import { MonthlySubscriptionSolana } from "../target/types/monthly_subscription_solana";
import { Enterprise } from "/mnt/c/Users/Mateo/monthly_subscription_Solana./monthly_subscription_Solana./tests/Accounts";
import { PublicKey } from '@solana/web3.js';

describe("Subscribe", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const wallet = provider.wallet as anchor.Wallet;
    const program = anchor.workspace.MonthlySubscriptionSolana as anchor.Program<MonthlySubscriptionSolana>;
  it('Creating subscription', async () => {
    const Account = await program.account.enterpriseData.fetch(Enterprise)
    const [userData, _bump] = await PublicKey.findProgramAddress(
      [
        Enterprise.toBuffer(),
        wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const tx = await program.methods.deleteAccount(
      "Mateo",
      "Lafalce"
    ).accounts({
        enterpriseData: Enterprise,
        userData: userData,
        user: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId
      }).rpc();
      console.log("---------------------------------------------")
      console.log("PDA: ", userData.toBase58())
      console.log("---------------------------------------------")
      console.log("Tx: ", tx);
      console.log("---------------------------------------------")
  });
});