import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MonthlySubscriptionSolana } from "../target/types/monthly_subscription_solana";
import { PDA } from "/mnt/c/Users/Mateo/monthly_subscription_solana/tests/Accounts";
import { PublicKey } from '@solana/web3.js';
const { SystemProgram } = anchor.web3;

describe("Subscribe", async () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const wallet = provider.wallet as anchor.Wallet;
    const program = anchor.workspace.MonthlySubscriptionSolana as Program<MonthlySubscriptionSolana>;
  it('Creating subscription', async () => {
    const [suscribeTo, _bump] = await PublicKey.findProgramAddress(
      [
        anchor.utils.bytes.utf8.encode("MyEnterprise-Suscribe"),
        wallet.publicKey.toBuffer(),
      ],
      program.programId
    )
    const tx = await program.methods.suscribe()
      .accounts({
        suscribeData: PDA,
        suscribeTo: suscribeTo,
        from: wallet.publicKey,
        stake: PDA,
        user: wallet.publicKey,
        systemProgram: SystemProgram.programId
      }).rpc();
      console.log("---------------------------------------------")
      console.log("Tx: ", tx);
      console.log("---------------------------------------------")
  });
});