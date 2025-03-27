import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProposalSystem } from "../target/types/proposal_system";
import { PublicKey, Keypair, SystemProgram } from "@solana/web3.js";
import { expect } from "chai";

describe("proposal-system", () => {});

describe("proposal-system", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.ProposalSystem as Program<ProposalSystem>;

  let multisig_program: Keypair;
  let multisig: Keypair;
  let proposal: Keypair;
  let creator: Keypair;
  let alice: Keypair;
  let bob: Keypair;
  let charlie: Keypair;
  let dave: Keypair;
  let eve: Keypair;
  let signers: Keypair[];
  let threshold: number;

  before(async () => {
    creator = Keypair.generate();
    alice = Keypair.generate();
    bob = Keypair.generate();
    charlie = Keypair.generate();
    dave = Keypair.generate();
    eve = Keypair.generate();
    signers = [creator, alice, bob];
    threshold = 2;
    multisig_program = Keypair.generate();
    multisig = Keypair.generate();
    proposal = Keypair.generate();

    await provider.connection.requestAirdrop(
      creator.publicKey, // signer needing SOL
      anchor.web3.LAMPORTS_PER_SOL * 2 // Airdrop 2 SOL
    );

    await provider.connection.requestAirdrop(
      alice.publicKey, // signer needing SOL
      anchor.web3.LAMPORTS_PER_SOL * 2 // Airdrop 2 SOL
    );

    await provider.connection.requestAirdrop(
      bob.publicKey, // signer needing SOL
      anchor.web3.LAMPORTS_PER_SOL * 2 // Airdrop 2 SOL
    );

    await provider.connection.requestAirdrop(
      charlie.publicKey, // signer needing SOL
      anchor.web3.LAMPORTS_PER_SOL * 2 // Airdrop 2 SOL
    );

    await provider.connection.requestAirdrop(
      dave.publicKey, // signer needing SOL
      anchor.web3.LAMPORTS_PER_SOL * 2 // Airdrop 2 SOL
    );

    await provider.connection.requestAirdrop(
      eve.publicKey, // signer needing SOL
      anchor.web3.LAMPORTS_PER_SOL * 2 // Airdrop 2 SOL
    );
  });

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts([
        {
          pubkey: multisig_program.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: creator.publicKey,
          isSigner: true,
          isWritable: true,
        },
        {
          pubkey: SystemProgram.programId,
          isSigner: false,
          isWritable: false,
        },
      ])
      .signers([creator, multisig_program])
      .instruction(); // Get the instruction without sending

    const transaction = new anchor.web3.Transaction().add(tx);
    transaction.feePayer = creator.publicKey;
    transaction.recentBlockhash = (
      await provider.connection.getLatestBlockhash()
    ).blockhash;

    // Manually sign and send
    transaction.sign(creator, multisig_program);

    const signature = await provider.connection.sendRawTransaction(
      transaction.serialize()
    );
    await provider.connection.confirmTransaction(signature);

    console.log("Transaction Signature:", signature);
    console.log("Your transaction signature", tx);
  });

  /**
   * ✅ TEST CASE: Create a multisig account successfully.
   */
  xit("Creates a multisig account", async () => {
    // Add your test here.
    const tx = await program.methods
      .createMultisig()
      .accounts([
        {
          multisig: multisig.publicKey,
          creator: creator.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
      ])
      .args([
        {
          name: "Team Wallet",
          signers: signers.map((s) => s.publicKey),
          threshold: new anchor.BN(threshold),
        },
      ]);
    console.log("Your transaction signature", tx);
    await program.rpc.createMultisig(
      "Team Wallet",
      signers.map((s) => s.publicKey),
      new anchor.BN(threshold),
      {
        accounts: {
          multisig: multisig.publicKey,
          creator: creator.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [multisig, creator],
      }
    );

    const multisigAccount = await program.account.multisig.fetch(
      multisig.publicKey
    );
    expect(multisigAccount.name).to.equal("Team Wallet");
    expect(multisigAccount.creator.toBase58()).to.equal(
      creator.publicKey.toBase58()
    );
    expect(multisigAccount.signers.length).to.equal(signers.length);
    expect(multisigAccount.threshold).to.equal(threshold);
  });

  /**
   * ✅ TEST CASE: Create a proposal successfully.
   */
  xit("Creates a proposal", async () => {
    await program.rpc.createProposal(
      { mint: { to: alice.publicKey, amount: new anchor.BN(100) } }, // Sample instruction
      {
        accounts: {
          multisig: multisig.publicKey,
          proposal: proposal.publicKey,
          creator: creator.publicKey,
          mint: alice.publicKey,
          systemProgram: SystemProgram.programId,
        },
        signers: [proposal, creator],
      }
    );

    const proposalAccount = await program.account.proposal.fetch(
      proposal.publicKey
    );
    expect(proposalAccount.multisig.toBase58()).to.equal(
      multisig.publicKey.toBase58()
    );
    expect(proposalAccount.creator.toBase58()).to.equal(
      creator.publicKey.toBase58()
    );
    expect(proposalAccount.executed).to.be.false;
  });

  /**
   * ✅ TEST CASE: Vote on a proposal.
   */
  xit("Votes on a proposal", async () => {
    await program.rpc.voteProposal({
      accounts: {
        multisig: multisig.publicKey,
        proposal: proposal.publicKey,
        signer: alice.publicKey,
      },
      signers: [alice],
    });

    const proposalAccount = await program.account.proposal.fetch(
      proposal.publicKey
    );
    expect(proposalAccount.votes).to.include.members([alice.publicKey]);
  });

  /**
   * ✅ TEST CASE: Execute a proposal successfully.
   */
  xit("Executes a proposal", async () => {
    await program.rpc.executeProposal({
      accounts: {
        multisig: multisig.publicKey,
        proposal: proposal.publicKey,
        executor: creator.publicKey,
        mint: alice.publicKey,
        mintAuthority: creator.publicKey,
        from: alice.publicKey, // since it is a mint the from account dosen't matter
        to: alice.publicKey,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      },
      signers: [creator],
    });

    const proposalAccount = await program.account.proposal.fetch(
      proposal.publicKey
    );
    expect(proposalAccount.executed).to.be.true;
  });
});
