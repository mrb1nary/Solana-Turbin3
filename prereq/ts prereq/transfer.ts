import {
  Transaction,
  SystemProgram,
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  sendAndConfirmTransaction,
  PublicKey,
} from "@solana/web3.js";

import wallet from "./dev-wallet.json";

const from = Keypair.fromSecretKey(Uint8Array.from(wallet));

const to = new PublicKey("GLtaTaYiTQrgz411iPJD79rsoee59HhEy18rtRdrhEUJ");

const connection = new Connection("https://api.devnet.solana.com");

async function transferSolToTurbin3() {
  try {
    const transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: LAMPORTS_PER_SOL / 100,
      })
    );
    transaction.recentBlockhash = (
      await connection.getLatestBlockhash("confirmed")
    ).blockhash;
    transaction.feePayer = from.publicKey;

    //Sign txn, broadcast and confirm
    const signature = await sendAndConfirmTransaction(connection, transaction, [
      from,
    ]);
    console.log(
      `Success!!! Checkout your TXN here: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
  } catch (e) {
    console.error(e);
  }
}

async function transferSolToMyself() {
  try {
    //Let's fetch the balance of the wallet
    const balance = await connection.getBalance(from.publicKey);

    //Creating a test txn so that we know how much fee/gas it would require
    const txn = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: from.publicKey,
        lamports: balance,
      })
    );
    txn.recentBlockhash = (
      await connection.getLatestBlockhash("confirmed")
    ).blockhash;
    txn.feePayer = from.publicKey;

    //Now we need to calculate exactly how much SOL we can transfer ourselves minus the fees

    const fee =
      (await connection.getFeeForMessage(txn.compileMessage(), "confirmed"))
        .value || 0;

    //Removing the transfer function that we used to calculate the fee
    txn.instructions.pop();

    //Now let's add the real instruction with exact amount of lamports

    txn.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: new PublicKey("5P5z8bf9bmZvXaQoQB93pci4LuarBMKwa4Uhnzbjc3gG"),
        lamports: balance - fee,
      })
    );
    //Sign txn, broadcast and confirm

    const signature = await sendAndConfirmTransaction(connection, txn, [from]);
    console.log(
      `Success!!! Checkout your TXN here: https://explorer.solana.com/tx/${signature}?cluster=devnet`
    );
  } catch (e) {
    console.error(e);
  }
}
// transferSolToTurbin3();
transferSolToMyself();
