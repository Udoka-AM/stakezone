import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stakezone } from "../target/types/stakezone";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint, createAssociatedTokenAccount, mintTo, getAssociatedTokenAddress, createAccount, getMinimumBalanceForRentExemptAccount } from "@solana/spl-token";
import { assert } from "chai";
import { getOrCreateATAInstruction } from "./helpers";

describe("stakezone", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Stakezone as Program<Stakezone>;
  const creator = Keypair.generate();
  const usdcMint = Keypair.generate();
  const feePayer = Keypair.generate();
  const participant = Keypair.generate();
  const rewardAuthority = Keypair.generate();

  before(async () => {
    // Airdrop SOL to creator, fee payer, and participant
    const signature = await provider.connection.requestAirdrop(creator.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature);
    const signature2 = await provider.connection.requestAirdrop(feePayer.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature2);
    const signature3 = await provider.connection.requestAirdrop(participant.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature3);
    const signature4 = await provider.connection.requestAirdrop(rewardAuthority.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.confirmTransaction(signature4);

    // Create USDC mint
    await createMint(
      provider.connection,
      creator,
      creator.publicKey,
      null,
      6,
      usdcMint
    );

    // Create participant's USDC account
    const participantUsdcAccount = await createAssociatedTokenAccount(
      provider.connection,
      participant,
      usdcMint.publicKey,
      participant.publicKey
    );

    // Mint USDC to participant
    await mintTo(
      provider.connection,
      participant,
      usdcMint.publicKey,
      participantUsdcAccount,
      creator,
      10000000 // 10 USDC
    );
  });

  it("Successfully initializes stake pool", async () => {
    const [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), creator.publicKey.toBuffer()],
      program.programId
    );

    const rewardTiers = [
      { threshold: 1, percentage: 50 },
      { threshold: 2, percentage: 30 },
      { threshold: 3, percentage: 20 }
    ];

    const startTime = Math.floor(Date.now() / 1000);
    const endTime = startTime + 86400; // 24 hours from now

    await program.methods
      .initializeStakePool(
        new anchor.BN(1000000), // 1 USDC entry fee
        rewardTiers,
        new anchor.BN(startTime),
        new anchor.BN(endTime)
      )
      .accountsStrict({
        creator: creator.publicKey,
        pool: poolPda,
        usdcMint: usdcMint.publicKey,
        feePayer: feePayer.publicKey,
        systemProgram: SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([creator])
      .rpc();

    const poolAccount = await program.account.pool.fetch(poolPda);
    assert.equal(poolAccount.creator.toString(), creator.publicKey.toString());
    assert.equal(poolAccount.entryFee.toString(), "1000000");
    assert.equal(poolAccount.startTime.toString(), startTime.toString());
    assert.equal(poolAccount.endTime.toString(), endTime.toString());
    assert.equal(poolAccount.participants, 0);
    assert.equal(poolAccount.isActive, true);
    assert.equal(poolAccount.rewardTiers.length, 3);
  });

  it("Successfully joins stake pool", async () => {
    const [poolPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool"), creator.publicKey.toBuffer()],
      program.programId
    );

    const [participationPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("participation"), poolPda.toBuffer(), participant.publicKey.toBuffer()],
      program.programId
    );

    const [poolUsdcPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("pool_usdc"), poolPda.toBuffer()],
      program.programId
    );

    // Create pool's USDC account
    const { ataPubKey: poolUsdcAccount, ix } = await getOrCreateATAInstruction(
      provider.connection,
      usdcMint.publicKey,
      rewardAuthority.publicKey,
      rewardAuthority.publicKey,
      true
    );
    if (ix) {
      const tx = new anchor.web3.Transaction();
      tx.add(ix);
      await provider.sendAndConfirm(tx, [rewardAuthority]);
    }

    const participantUsdcAccount = await getAssociatedTokenAddress(
      usdcMint.publicKey,
      participant.publicKey
    );

    await program.methods
      .joinStakePool(1) // FPL team ID 1
      .accountsStrict({
        participant: participant.publicKey,
        pool: poolPda,
        usdcMint: usdcMint.publicKey,
        participation: participationPda,
        participantUsdcAccount: participantUsdcAccount,
        poolUsdcAccount: poolUsdcAccount,
        rewardAuthority: rewardAuthority.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([participant, rewardAuthority])
      .rpc();

    const participationAccount = await program.account.participation.fetch(participationPda);
    assert.equal(participationAccount.pool.toString(), poolPda.toString());
    assert.equal(participationAccount.participant.toString(), participant.publicKey.toString());
    assert.equal(participationAccount.fplTeamId.toString(), "1");
    assert.equal(participationAccount.score, 0);

    const poolAccount = await program.account.pool.fetch(poolPda);
    assert.equal(poolAccount.participants, 1);
  });
});
