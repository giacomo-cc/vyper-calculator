import { Keypair, PublicKey } from "@solana/web3.js";
import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { VyperCalculator } from "../target/types/vyper_calculator";
import { assert, expect } from "chai";

describe("vyper-calculator", () => {
  const provider = anchor.AnchorProvider.env();
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const program = anchor.workspace.VyperCalculator as Program<VyperCalculator>;

  // * * * * * * * * * * * * * *
  // SUM

  it("sum", async () => {
    const statePubkey = await initializeState(program);

    // op
    const inputA = 30;
    const inputB = 10;
    await program.methods
      .sum(bn(inputA), bn(inputB))
      .accounts({
        state: statePubkey,
      })
      .rpc();

    // account fetch
    const stateAccountInfo = await program.account.state.fetch(statePubkey);
    expect(stateAccountInfo.value.toNumber()).to.eq(inputA + inputB);
  });

  // * * * * * * * * * * * * * *
  // SUB

  it("sub", async () => {
    const statePubkey = await initializeState(program);

    // op
    const inputA = 30;
    const inputB = 10;
    await program.methods
      .sub(bn(inputA), bn(inputB))
      .accounts({
        state: statePubkey,
      })
      .rpc();

    // account fetch
    const stateAccountInfo = await program.account.state.fetch(statePubkey);
    expect(stateAccountInfo.value.toNumber()).to.eq(inputA - inputB);
  });

  // * * * * * * * * * * * * * *
  // MUL

  it("mul", async () => {
    const statePubkey = await initializeState(program);

    // op
    const inputA = 30;
    const inputB = 10;
    await program.methods
      .mul(bn(inputA), bn(inputB))
      .accounts({
        state: statePubkey,
      })
      .rpc();

    // account fetch
    const stateAccountInfo = await program.account.state.fetch(statePubkey);
    expect(stateAccountInfo.value.toNumber()).to.eq(inputA * inputB);
  });

  // * * * * * * * * * * * * * *
  // DIV

  it("div", async () => {
    const statePubkey = await initializeState(program);

    // op
    const inputA = 30;
    const inputB = 10;
    await program.methods
      .div(bn(inputA), bn(inputB))
      .accounts({
        state: statePubkey,
      })
      .rpc();

    // account fetch
    const stateAccountInfo = await program.account.state.fetch(statePubkey);
    expect(stateAccountInfo.value.toNumber()).to.eq(inputA / inputB);
  });
});

async function initializeState(program: anchor.Program<VyperCalculator>): Promise<PublicKey> {
  const calStateKp = Keypair.generate();
  await program.methods
    .initialize()
    .accounts({
      state: calStateKp.publicKey,
    })
    .signers([calStateKp])
    .rpc();
  return calStateKp.publicKey;
}

function bn(val: number): anchor.BN {
  return new anchor.BN(val);
}
