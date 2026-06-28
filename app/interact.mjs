import { readFileSync } from "fs";
import { homedir } from "os";
import { createRequire } from "module";
import { Keypair, Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";

const require = createRequire(import.meta.url);
const IDL = require("../target/idl/solana_counter.json");

const RPC_URL =
  process.env.SOLANA_RPC_URL || "https://api.devnet.solana.com";
const walletPath =
  process.env.ANCHOR_WALLET ||
  `${homedir()}/workspaces/ws-personal/wallet.json`;

const secretKey = JSON.parse(readFileSync(walletPath, "utf-8"));
const payer = Keypair.fromSecretKey(Uint8Array.from(secretKey));
const wallet = new Wallet(payer);

const connection = new Connection(RPC_URL, "confirmed");
const provider = new AnchorProvider(connection, wallet, {
  commitment: "confirmed",
});
const program = new Program(IDL, provider);

const [counterPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("counter")],
  program.programId
);

console.log("program: ", program.programId.toBase58());
console.log("counter: ", counterPda.toBase58());
console.log("payer:   ", payer.publicKey.toBase58());

console.log("\n[initialize]");
try {
  const sig = await program.methods.initialize().rpc();
  console.log(`https://explorer.solana.com/tx/${sig}?cluster=devnet`);
} catch (e) {
  const msg = e.toString();
  if (msg.includes("already in use") || msg.includes("0x0")) {
    console.log("  (already initialized, skipping)");
  } else {
    throw e;
  }
}

console.log("\n[increment]");
const incrSig = await program.methods.increment().rpc();
console.log(`https://explorer.solana.com/tx/${incrSig}?cluster=devnet`);

const state = await program.account.counter.fetch(counterPda);
console.log("\ncount =", state.count.toString());
