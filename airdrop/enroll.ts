import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";
import wallet from "./Turbin3-wallet.json";
import bs58 from "bs58";

// const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const secretKey = bs58.decode(wallet);
const keypair = Keypair.fromSecretKey(secretKey);

console.log("Public key:", keypair.publicKey.toString());

const connection = new Connection("https://api.devnet.solana.com");
const github = Buffer.from("mrb1nary", "utf-8");

const provider = new AnchorProvider(connection, new Wallet(keypair), {
  commitment: "confirmed",
});

const program: Program<Turbin3Prereq> = new Program(IDL, provider);

//Re-read this thing again later
const enrollment_seeds = [Buffer.from("pre"), keypair.publicKey.toBuffer()];

const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(
  enrollment_seeds,
  program.programId
);

async function enrollFunction() {
  try {
    const txHash = await program.methods
      .submit(github)
      .accounts({
        signer: keypair.publicKey,
      })
      .signers([keypair])
      .rpc();
    console.log(`Success! Check out your TX here:
        https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
  } catch (e) {
    console.error(e);
  }
}

enrollFunction();
