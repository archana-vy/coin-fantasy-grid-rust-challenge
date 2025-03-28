import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ProposalSystem } from "../target/types/proposal_system";
import { PublicKey, Keypair } from "@solana/web3.js";
import { expect } from "chai";
import {
  createMint,
  getAssociatedTokenAddress,
  mintTo,
  getAccount,
  createAssociatedTokenAccountInstruction,
  createAssociatedTokenAccount,
} from "@solana/spl-token";

describe("proposal-system", () => {});

describe("proposal-system", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.ProposalSystem as Program<ProposalSystem>;

  let multisig_program: Keypair;
  let multisig: Keypair;
  let proposal: Keypair;

  let mint: Keypair;
  let mintPubkey: PublicKey;
  let tokenAccountPubkey: PublicKey;

  let creator: Keypair;
  let alice: Keypair;
  let bob: Keypair;
  let charlie: Keypair;
  let dave: Keypair;
  let eve: Keypair;

  let signers: Keypair[];
  let threshold: number;

  let creatorATA: PublicKey;
  let aliceATA: PublicKey;
  let bobATA: PublicKey;
  let charlieATA: PublicKey;
  let daveATA: PublicKey;
  let eveATA: PublicKey;

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

    mint = Keypair.generate();

    let tx = await provider.connection.requestAirdrop(
      creator.publicKey, // signer needing SOL
      anchor.web3.LAMPORTS_PER_SOL * 20 // Airdrop 20 SOL
    );
    await provider.connection.confirmTransaction(tx, "confirmed");
    const balance = await provider.connection.getBalance(creator.publicKey);
    console.log("Creator SOL Balance:", balance / anchor.web3.LAMPORTS_PER_SOL);

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

    // Step 1: Create a new SPL Token Mint

    mintPubkey = await createMint(
      provider.connection, // Connection to Solana
      creator, // Payer
      creator.publicKey, // Mint Authority
      null, // Freeze Authority (optional)
      6 // Decimals
    );

    console.log("Mint created:", mintPubkey.toBase58());

    try {
      // Step 2: Create the Associated Token Account for the User

      creatorATA = await createAssociatedTokenAccount(
        provider.connection,
        creator, // Payer
        mintPubkey, // Mint Address
        creator.publicKey // Owner
      );
      creatorATA = await getAssociatedTokenAddress(
        mintPubkey,
        creator.publicKey
      );

      console.log("Creator Token Account:", creatorATA.toBase58());

      // Step 3: Mint tokens to the User's Associated Token Account
      await mintTo(
        provider.connection,
        creator,
        mintPubkey,
        creatorATA,
        creator.publicKey, // Mint authority
        1000000000 // Amount to mint (in smallest unit)
      );

      // Step 2: Create an Associated Token Account for the User

      aliceATA = await createAssociatedTokenAccount(
        provider.connection,
        alice, // Payer
        mintPubkey, // Mint Address
        alice.publicKey // Owner
      );

      aliceATA = await getAssociatedTokenAddress(mintPubkey, alice.publicKey);

      console.log("Alice Token Account:", aliceATA.toBase58());

      // Step 3: Mint tokens to the User's Associated Token Account
      await mintTo(
        provider.connection,
        creator,
        mintPubkey,
        aliceATA,
        creator.publicKey, // Mint authority
        1000000000 // Amount to mint (in smallest unit)
      );

      // Step 2: Create an Associated Token Account for the User

      bobATA = await createAssociatedTokenAccount(
        provider.connection,
        bob, // Payer
        mintPubkey, // Mint Address
        bob.publicKey // Owner
      );
      bobATA = await getAssociatedTokenAddress(mintPubkey, bob.publicKey);

      console.log("Bob Token Account:", bobATA.toBase58());

      // Step 3: Mint tokens to the User's Associated Token Account
      await mintTo(
        provider.connection,
        creator,
        mintPubkey,
        bobATA,
        creator.publicKey, // Mint authority
        1000000000 // Amount to mint (in smallest unit)
      );

      // Step 2: Create an Associated Token Account for the User

      charlieATA = await createAssociatedTokenAccount(
        provider.connection,
        charlie, // Payer
        mintPubkey, // Mint Address
        charlie.publicKey // Owner
      );
      charlieATA = await getAssociatedTokenAddress(
        mintPubkey,
        charlie.publicKey
      );

      console.log("Charlie Token Account:", charlieATA.toBase58());

      // Step 3: Mint tokens to the User's Associated Token Account
      await mintTo(
        provider.connection,
        creator,
        mintPubkey,
        charlieATA,
        creator.publicKey, // Mint authority
        1000000000 // Amount to mint (in smallest unit)
      );

      // Step 2: Create an Associated Token Account for the User

      daveATA = await createAssociatedTokenAccount(
        provider.connection,
        dave, // Payer
        mintPubkey, // Mint Address
        dave.publicKey // Owner
      );
      daveATA = await getAssociatedTokenAddress(mintPubkey, dave.publicKey);

      console.log("Dave Token Account:", daveATA.toBase58());

      // Step 3: Mint tokens to the User's Associated Token Account
      await mintTo(
        provider.connection,
        creator,
        mintPubkey,
        daveATA,
        creator.publicKey, // Mint authority
        1000000000 // Amount to mint (in smallest unit)
      );

      // Step 2: Create an Associated Token Account for the User

      eveATA = await createAssociatedTokenAccount(
        provider.connection,
        eve, // Payer
        mintPubkey, // Mint Address
        eve.publicKey // Owner
      );
      eveATA = await getAssociatedTokenAddress(mintPubkey, eve.publicKey);

      console.log("Eve Token Account:", eveATA.toBase58());

      // Step 3: Mint tokens to the User's Associated Token Account
      await mintTo(
        provider.connection,
        creator,
        mintPubkey,
        eveATA,
        creator.publicKey, // Mint authority
        1000000000 // Amount to mint (in smallest unit)
      );

      console.log("Minted tokens successfully");
    } catch (err) {
      console.log(err);
    }
  });

  xit("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        multisigProgram: multisig_program.publicKey,
        signer: creator.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([creator, multisig_program])
      .rpc();
    console.log("Your transaction signature", tx);
  });

  /**
   * ✅ TEST CASE: Create a multisig account successfully.
   */
  it("Creates a multisig account", async () => {
    // Add your test here.
    let signersPubKeys: PublicKey[] = signers.map((s) => s.publicKey);
    const tx = await program.methods
      .createMultisig("Team Wallet", signersPubKeys, threshold)
      .accounts({
        multisig: multisig.publicKey,
        creator: creator.publicKey,
        // systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([creator, multisig])
      .rpc();
    console.log("Your transaction signature", tx);

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
  it("Creates a proposal", async () => {
    await program.methods
      .createProposal({
        mint: { to: dave.publicKey, amount: new anchor.BN(100) },
      })
      .accounts({
        multisig: multisig.publicKey,
        proposal: proposal.publicKey,
        creator: creator.publicKey,
        mint: mintPubkey,
      })
      .signers([creator, proposal])
      .rpc();

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
  it("Votes on a proposal", async () => {
    await program.methods
      .voteProposal()
      .accounts({
        multisig: multisig.publicKey,
        proposal: proposal.publicKey,
        signer: alice.publicKey,
      })
      .signers([alice])
      .rpc();

    await program.methods
      .voteProposal()
      .accounts({
        multisig: multisig.publicKey,
        proposal: proposal.publicKey,
        signer: bob.publicKey,
      })
      .signers([bob])
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(
      proposal.publicKey
    );
    expect(proposalAccount.votes[0].toBase58()).to.eq(
      alice.publicKey.toBase58()
    );

    expect(proposalAccount.votes[1].toBase58()).to.eq(bob.publicKey.toBase58());
  });

  /**
   * ✅ TEST CASE: Execute a proposal successfully.
   */
  it("Executes a proposal", async () => {
    await program.methods
      .executeProposal()
      .accounts({
        multisig: multisig.publicKey,
        proposal: proposal.publicKey,
        executor: creator.publicKey,
        mint: mintPubkey,
        mintAuthority: creator.publicKey,
        from: null,
        fromAta: null,
        to: dave.publicKey,
        toAta: daveATA,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .signers([creator])
      .rpc();

    const proposalAccount = await program.account.proposal.fetch(
      proposal.publicKey
    );
    expect(proposalAccount.executed).to.be.true;
  });
});
