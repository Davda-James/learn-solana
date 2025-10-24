import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import assert from "assert";

describe("counter", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  
  const program = anchor.workspace.counter as Program<Counter>;
  const counter_keypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().accounts({
      user: program.provider.wallet.publicKey,
      counter: counter_keypair.publicKey,
    }).
    signers([counter_keypair])
    .rpc();
    console.log("transaction after initialization is : ",tx);
  });
  it("check increment", async() => {
    const tx = await program.methods.increment().accounts({
      counter: counter_keypair.publicKey,
    }).rpc();
    const value = (await program.account.counter.fetch(counter_keypair.publicKey)).value;
    assert.equal(value, 1);
  })
  it("check decrement", async() => {
    const tx = await program.methods.decrement().accounts({
      counter: counter_keypair.publicKey,
    }).rpc();
    const value = (await program.account.counter.fetch(counter_keypair.publicKey)).value;
    assert.equal(value, 0);
  })
});
