import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiIntoNativeContract } from "../target/types/cpi_into_native_contract";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";

describe("cpi-into-native-contract", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .cpiIntoNativeContract as Program<CpiIntoNativeContract>;

  const systemProgramId = anchor.web3.SystemProgram.programId;

  const dataAccount = anchor.web3.Keypair.generate();

  const nativeProgramId = new anchor.web3.PublicKey(
    "D368GCzmCrfjFBEkvWgC1WZ9yKmCmV15ec2nrBADwzfg"
  );

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .init(0)
      .accounts({
        dataAccount: dataAccount.publicKey,
        userAccount: anchor.getProvider().wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        cpiProgram: nativeProgramId,
      })
      .signers([dataAccount])
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
