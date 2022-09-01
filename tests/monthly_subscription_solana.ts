import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MonthlySubscriptionSolana } from "../target/types/monthly_subscription_solana";

describe("monthly_subscription_solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.MonthlySubscriptionSolana as Program<MonthlySubscriptionSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
