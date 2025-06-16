import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { build } from "bun";
import { beforeAll, describe, expect, test } from "bun:test";
import { LiteSVM } from "litesvm";

describe("Counter Program Tests", () => {
  let svm: LiteSVM;
  let program_id: PublicKey;
  let dataAccount: Keypair;
  let userAccount: Keypair;

  beforeAll(() => {
    svm = new LiteSVM();

    program_id = PublicKey.unique();

    svm.addProgramFromFile(program_id, "./cpi-native-contract-2.so");

    dataAccount = new Keypair();

    userAccount = new Keypair();

    svm.airdrop(userAccount.publicKey, BigInt(LAMPORTS_PER_SOL));
  });

  test("initialize counter", async () => {
    const ix = new TransactionInstruction({
      programId: program_id,
      keys: [
        { pubkey: dataAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: userAccount.publicKey, isSigner: true, isWritable: true },
        { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      data: Buffer.from([0]),
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.feePayer = userAccount.publicKey;
    tx.sign(dataAccount, userAccount);
    svm.sendTransaction(tx);
    svm.expireBlockhash();

    const updatedDataAccount = svm.getAccount(dataAccount.publicKey);

    if (!updatedDataAccount) {
      throw new Error("Account not found");
    }
    console.log(updatedDataAccount.data);

    expect(updatedDataAccount.data[3]).toBe(0);
    expect(updatedDataAccount.data[0]).toBe(1);
    expect(updatedDataAccount.data[1]).toBe(0);
    expect(updatedDataAccount.data[2]).toBe(0);
  });

  test("double counter", async () => {
    function doubleCounter() {
      let ix = new TransactionInstruction({
        programId: program_id,
        keys: [
          { pubkey: dataAccount.publicKey, isSigner: true, isWritable: true },
        ],
        data: Buffer.from([1]),
      });

      let tx = new Transaction().add(ix);
      tx.recentBlockhash = svm.latestBlockhash();
      tx.feePayer = userAccount.publicKey;
      tx.sign(userAccount, dataAccount);
      let txn = svm.sendTransaction(tx);
      svm.expireBlockhash();
    }

    doubleCounter();
    doubleCounter();
    doubleCounter();
    doubleCounter();

    const updatedDataAccount = svm.getAccount(dataAccount.publicKey);
    if (!updatedDataAccount) {
      throw new Error("Account not found");
    }
    console.log(updatedDataAccount.data);

    expect(updatedDataAccount.data[0]).toBe(16);
    expect(updatedDataAccount.data[1]).toBe(0);
    expect(updatedDataAccount.data[2]).toBe(0);
    expect(updatedDataAccount.data[3]).toBe(0);
  });

  test("half counter", async () => {
    function halfCounter() {
      let ix = new TransactionInstruction({
        programId: program_id,
        keys: [
          { pubkey: dataAccount.publicKey, isSigner: true, isWritable: true },
        ],
        data: Buffer.from([2]),
      });

      let tx = new Transaction().add(ix);
      tx.recentBlockhash = svm.latestBlockhash();
      tx.feePayer = userAccount.publicKey;
      tx.sign(userAccount, dataAccount);
      let txn = svm.sendTransaction(tx);
      svm.expireBlockhash();
    }

    halfCounter();
    halfCounter();

    const updatedDataAccount = svm.getAccount(dataAccount.publicKey);
    if (!updatedDataAccount) {
      throw new Error("Account not found");
    }
    console.log(updatedDataAccount.data);

    expect(updatedDataAccount.data[0]).toBe(4);
    expect(updatedDataAccount.data[1]).toBe(0);
    expect(updatedDataAccount.data[2]).toBe(0);
    expect(updatedDataAccount.data[3]).toBe(0);
  });
});
