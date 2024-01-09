import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { MonthlySubscriptionSolana } from "../target/types/monthly_subscription_solana";
import { SystemProgram, PublicKey } from "@solana/web3.js";

describe("Register a business", () => {
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

    const tx = await program.methods
      .createEnterprise(new anchor.BN(23), "Gym 24/7")
      .accounts({
        enterpriseData: enterpriseData,
        user: payer.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();
    console.log("Transaction signature", tx);
  });
});
