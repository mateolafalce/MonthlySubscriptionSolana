import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MonthlySubscriptionSolana } from "../target/types/monthly_subscription_solana";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Use suscription", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const payer = provider.wallet as anchor.Wallet;
  const program = anchor.workspace
    .MonthlySubscriptionSolana as Program<MonthlySubscriptionSolana>;
  const programId = program.programId;

  it("Is initialized!", async () => {
    const enterpriseData = PublicKey.findProgramAddressSync(
      [Buffer.from("Enterprise", "utf8"), payer.publicKey.toBuffer()],
      programId
    )[0];

    const userData = PublicKey.findProgramAddressSync(
      [enterpriseData.toBuffer(), payer.publicKey.toBuffer()],
      programId
    )[0];

    const tx = await program.methods
      .deleteAccount()
      .accounts({
        enterpriseData: enterpriseData,
        userData: userData,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
