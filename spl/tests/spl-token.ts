import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplToken } from "../target/types/spl_token";
import { assert } from "chai";

describe("spl-token", async () => {
  const LAMPORTS_PER_SOL = anchor.web3.LAMPORTS_PER_SOL;

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.splToken as Program<SplToken>;

  const aliceKeypair = anchor.web3.Keypair.generate();
  const mintKeypair = anchor.web3.Keypair.generate();
  const bobKeypair = anchor.web3.Keypair.generate();
  const aliceTokenAccountKeypair = anchor.web3.Keypair.generate();
  const bobTokenAccountKeypair = anchor.web3.Keypair.generate();
  
  it("initializing mint", async () => {
    await airdrop(provider.connection, aliceKeypair.publicKey);
    // console.log(await provider.connection.getBalance(aliceKeypair.publicKey) / LAMPORTS_PER_SOL);

    const txSig = await program.methods.initialize(6)
    .accounts({
      mint: mintKeypair.publicKey,
      owner: aliceKeypair.publicKey,
    })
    .signers([mintKeypair, aliceKeypair]) 
    .rpc({commitment: "confirmed"});
    // console.log("Transaction signature", txSig);
    
    // const txParsed = await provider.connection.getParsedTransaction(txSig, "confirmed");
    // // console.log("Transaction parsed", txParsed.meta.logMessages);
    // const eventParse = new anchor.EventParser(program.programId, program.coder);
    // const events = eventParse.parseLogs(txParsed.meta.logMessages);

    // let logEmitted = false;
    // for (let event of events) {
    //   if (event.name === "MintInitialized") {
    //     logEmitted = true;
    //     assert.strictEqual(event.data.mint.toBase58(), mintKeypair.publicKey.toBase58());
    //     assert.strictEqual(event.data.mint_authority.toBase58(), aliceKeypair.publicKey.toBase58());
    //     assert.strictEqual(event.data.freeze_authority.toBase58(), aliceKeypair.publicKey.toBase58());
    //     assert.strictEqual(event.data.decimals, 6);
    //   }
    // }
    // assert.equal(logEmitted, true, "MintInitialized event not emitted")
  });

  it("alice and bob token account initialized", async()=> {
      await airdrop(provider.connection, aliceKeypair.publicKey);
      await airdrop(provider.connection, bobKeypair.publicKey);

      // 2 bond tokens (as decimal is 6)
      const aliceTxSig = await program.methods.initTokenAccount(new anchor.BN(4000000))
      .accounts({
        tokenAccount: aliceTokenAccountKeypair.publicKey,
        owner: aliceKeypair.publicKey,
        mint: mintKeypair.publicKey,
      })
      .signers([aliceKeypair, aliceTokenAccountKeypair])
      .rpc({ commitment: "confirmed"});

      const bobTxSig = await program.methods.initTokenAccount(new anchor.BN(4000000))
      .accounts({
        tokenAccount: bobTokenAccountKeypair.publicKey,
        owner: bobKeypair.publicKey,
        mint: mintKeypair.publicKey,
      })
      .signers([bobKeypair, bobTokenAccountKeypair])
      .rpc({ commitment: "confirmed"});
      
      // const aliceTx = await provider.connection.getParsedTransaction(aliceTxSig, "confirmed");
      // const bobTx = await provider.connection.getParsedTransaction(bobTxSig, "confirmed");  

      // const eventParse = new anchor.EventParser(program.programId, program.coder);
      
      // const aliceEvents = eventParse.parseLogs(aliceTx.meta.logMessages);
      // const bobEvents = eventParse.parseLogs(bobTx.meta.logMessages);
      // console.log(aliceEvents);

      const aliceTokenAccnt = await program.account.tokenAccount.fetch(aliceTokenAccountKeypair.publicKey);
      const bobTokenAccnt = await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey);
      const supplyAfter = (await program.account.mint.fetch(mintKeypair.publicKey)).supply; 
      assert.strictEqual(aliceTokenAccnt.amount.toNumber(), 4000000);
      assert.strictEqual(bobTokenAccnt.amount.toNumber(), 4000000);
      assert.strictEqual(supplyAfter.toNumber(), 8000000);
    })

    it("transfer tokens from alice to bob", async() => {
    const aliceBalanceBefore = (await program.account.tokenAccount.fetch(aliceTokenAccountKeypair.publicKey)).amount;
    const bobBalanceBefore = (await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey)).amount;
    const txSig = await program.methods.transfer(new anchor.BN(1000000))
      .accounts({
        from: aliceTokenAccountKeypair.publicKey,
        to: bobTokenAccountKeypair.publicKey,
        owner: aliceKeypair.publicKey,
      })
      .signers([aliceKeypair])  
      .rpc({ commitment: "confirmed"})

    const tx = await provider.connection.getParsedTransaction(txSig, "confirmed");
    const eventParser = new anchor.EventParser(program.programId, program.coder);
    const events = eventParser.parseLogs(tx.meta.logMessages);
    const aliceBalanceAfter = await program.account.tokenAccount.fetch(aliceTokenAccountKeypair.publicKey);
    const bobBalanceAfter = await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey);

    assert.strictEqual(aliceBalanceAfter.amount.toNumber(), aliceBalanceBefore.toNumber() - 1000000);
    assert.strictEqual(bobBalanceAfter.amount.toNumber(), bobBalanceBefore.toNumber() + 1000000);
    })

    it("froze the bob token", async() => {
      const txSig = await program.methods.freeze()
      .accounts({
        mint: mintKeypair.publicKey,
        freezeAuthority: aliceKeypair.publicKey,
        tokenAccount: bobTokenAccountKeypair.publicKey,
      })
      .signers([aliceKeypair])
      .rpc({ commitment: "confirmed"}); 

      const frozen_accnt = await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey);
      assert.strictEqual(frozen_accnt.frozen, true);
    })

    it("transfer from frozen account should fail", async() => {
      // transfer 1 bond token
      try{ 
        const txSig = await program.methods.transfer(new anchor.BN(1000000))
          .accounts({
            from: bobTokenAccountKeypair.publicKey,
            to: aliceTokenAccountKeypair.publicKey,
            owner: bobKeypair.publicKey,
          })
          .signers([bobKeypair])  
          .rpc({ commitment: "confirmed"})
        assert.fail("Transfer from frozen account did not fail");
      } catch (err) {
        assert.include(err.message, "FrozenAccount");
      }
    })

    it("unfreeze the bob token", async() => {
      await program.methods.unfreeze()
      .accounts({
        mint: mintKeypair.publicKey,
        freezeAuthority: aliceKeypair.publicKey,
        tokenAccount: bobTokenAccountKeypair.publicKey
      })
      .signers([aliceKeypair])
      .rpc({ commitment: "confirmed"});
      const frozen_accnt = await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey);
      assert.strictEqual(frozen_accnt.frozen, false);
    })

    it("burn tokens from bob account", async ()=> {
      // should unfreeze first if frozen to burn some tokens
      if ((await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey)).frozen) {
        await program.methods.unfreeze()
        .accounts({
          mint: mintKeypair.publicKey,
          tokenAccount: bobTokenAccountKeypair.publicKey,
          freezeAuthority: aliceKeypair.publicKey,
        })
        .signers([aliceKeypair])
        .rpc({ commitment: "confirmed"}); 
      }
      const bobBalanceBeforeBurn = (await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey)).amount;
      const beforeBurn = (await program.account.mint.fetch(mintKeypair.publicKey)).supply;
      const txSig = await program.methods.burnTokens(new anchor.BN(50000))
      .accounts({
        from: bobTokenAccountKeypair.publicKey,
        mint: mintKeypair.publicKey,
        owner: bobKeypair.publicKey,
      })
      .signers([bobKeypair])
      .rpc({ commitment: "confirmed"});

      const mint = await program.account.mint.fetch(mintKeypair.publicKey);
      const bobTokenAccnt = await program.account.tokenAccount.fetch(bobTokenAccountKeypair.publicKey);
      assert.strictEqual(mint.supply.toNumber(), beforeBurn.toNumber() - 50000);
      assert.strictEqual(bobTokenAccnt.amount.toNumber(), bobBalanceBeforeBurn.toNumber() - 50000);
    })
});

async function airdrop(connection: any, address: any, amount = 100 * anchor.web3.LAMPORTS_PER_SOL) {
  await connection.confirmTransaction(await connection.requestAirdrop(address, amount), "confirmed");
}