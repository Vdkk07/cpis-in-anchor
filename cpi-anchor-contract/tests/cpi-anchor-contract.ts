import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiAnchorContract } from "../target/types/cpi_anchor_contract";
import { assert } from "chai";

describe("cpi-anchor-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .cpiAnchorContract as Program<CpiAnchorContract>;
  console.log("Program", program.programId);

  const recipient = anchor.web3.Keypair.generate();

  it("Is Sol Transfer!", async () => {
    // Add your test here.
    const tx = await program.methods
      .solTransfer(new anchor.BN(1000000000))
      .accounts({
        sender: anchor.getProvider().wallet.publicKey,
        recipient: recipient.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    const account = anchor
      .getProvider()
      .connection.getAccountInfo(recipient.publicKey);
    assert.equal((await account).lamports, 1000000000);
  });
});
