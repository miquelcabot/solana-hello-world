import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaHelloWorld } from "../target/types/solana_hello_world";

import chai from "chai";
import { expect } from "chai";

import chaiAsPromised from "chai-as-promised";
chai.use(chaiAsPromised);

// helper functions
const createWallet = async (connection: anchor.web3.Connection, funds: number) 
: Promise<anchor.web3.Keypair> => {
  const wallet = anchor.web3.Keypair.generate();
  const tx = await connection.requestAirdrop(
    wallet.publicKey,
    anchor.web3.LAMPORTS_PER_SOL * funds
  );
  // wait for confirmation
  const latestBlockHash = await connection.getLatestBlockhash();
  await connection.confirmTransaction({
    blockhash: latestBlockHash.blockhash,
    lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
    signature: tx
  });

  // check balance
  const balance = await connection.getBalance(wallet.publicKey);
  if(balance < funds){
    throw new Error("Requested amount exceeds target"+
                    "network's airdrop limit.");
  }
  return wallet;
};

describe("solana-hello-world", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SolanaHelloWorld as Program<SolanaHelloWorld>;

  it("Is initialized!", async () => {
    // Add your test here.
    // const tx = await program.methods.initialize().rpc();
    console.log("Uep");
    // console.log("Your transaction signature", tx);
  });
});
