import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { MvpContract } from "../target/types/mvp_contract";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import assert from "assert";

describe("mvp-contract", () => {
  // configure the client to use the local cluster.
  const provider = anchor.Provider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.MvpContract as Program<MvpContract>;

  let _baseAccount: anchor.web3.Keypair;
  // let _treasury: PublicKey = new PublicKey("66hNdzKJU2XWdDVZUkqfYtdnNBV3gjS8vC4QH5QDv9gE");
  let _price: number = 1;
  let _slots: number = 50;
  const baseAccount = anchor.web3.Keypair.generate();


  it("creates a base account raritybox", async () => {
    // call the initialize function via RPC
    const tx = await program.rpc.initialize(
      new anchor.BN(_price),
      new anchor.BN(_slots),
      {
        accounts: {
          baseAccount: baseAccount.publicKey,
          user: provider.wallet.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [baseAccount],
      }
    );

    const account = await program.account.baseAccount.fetch(
      baseAccount.publicKey
    );
    assert.equal(account.filled.toString(), "0");

    _baseAccount = baseAccount;
  });

  it("deliver ${price} SOL to ${treasury}", async () => {
    const user = anchor.web3.Keypair.generate();

    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(user.publicKey, 1000000),
      "confirmed"
    );

    const tx = await program.rpc.subscribe({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: user.publicKey,
        systemProgram: SystemProgram.programId,
      },
        signers: [user],
    });

  });
});
