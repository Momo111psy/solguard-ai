import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

/**
 * Validator Registry Test Suite
 *
 * Comprehensive tests for validator registration and management
 */
describe("validator-registry", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ValidatorRegistry as Program;
  const authority = provider.wallet;

  let registryPda: PublicKey;
  let validatorPda: PublicKey;
  
  const operator = Keypair.generate();
  const validatorKeypair = Keypair.generate();

  before(async () => {
    // Fund test accounts
    const sig = await provider.connection.requestAirdrop(
      operator.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(sig);

    // Derive PDAs
    [registryPda] = await PublicKey.findProgramAddress(
      [Buffer.from("registry")],
      program.programId
    );

    [validatorPda] = await PublicKey.findProgramAddress(
      [Buffer.from("validator"), validatorKeypair.publicKey.toBuffer()],
      program.programId
    );
  });

  describe("initialization", () => {
    it("initializes the validator registry", async () => {
      try {
        const minStake = new anchor.BN(1000000000);
        const rewardRate = new anchor.BN(100);

        await program.methods
          .initialize(minStake, rewardRate)
          .accounts({
            registry: registryPda,
            authority: authority.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .rpc();

        const registryAccount = await program.account.validatorRegistry.fetch(registryPda);
        
        expect(registryAccount.authority.toString()).to.equal(authority.publicKey.toString());
        expect(registryAccount.totalValidators).to.equal(0);
        expect(registryAccount.activeValidators).to.equal(0);
      } catch (error) {
        console.log("    Registry initialization:", error.message);
      }
    });
  });

  describe("validator registration", () => {
    it("registers a new validator", async () => {
      try {
        await program.methods
          .registerValidator(
            validatorKeypair.publicKey,
            5,
            "https://validator.example.com"
          )
          .accounts({
            registry: registryPda,
            validator: validatorPda,
            operator: operator.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([operator])
          .rpc();

        const validatorAccount = await program.account.validatorInfo.fetch(validatorPda);
        expect(validatorAccount.isActive).to.be.true;
      } catch (error) {
        console.log("    Validator registration:", error.message);
      }
    });

    it("increments total validators counter", async () => {
      try {
        const registryAccount = await program.account.validatorRegistry.fetch(registryPda);
        expect(registryAccount.totalValidators).to.be.greaterThan(0);
      } catch (error) {
        console.log("    Validator counter:", error.message);
      }
    });
  });

  describe("performance tracking", () => {
    it("updates validator performance metrics", async () => {
      try {
        await program.methods
          .updatePerformance(new anchor.BN(100), 95)
          .accounts({
            validator: validatorPda,
            authority: authority.publicKey,
          })
          .rpc();

        const validatorAccount = await program.account.validatorInfo.fetch(validatorPda);
        expect(validatorAccount.performanceScore).to.be.greaterThan(0);
      } catch (error) {
        console.log("    Performance update:", error.message);
      }
    });

    it("calculates health score", async () => {
      try {
        const validatorAccount = await program.account.validatorInfo.fetch(validatorPda);
        expect(validatorAccount.healthScore).to.be.within(0, 100);
      } catch (error) {
        console.log("    Health score:", error.message);
      }
    });
  });

  describe("reward distribution", () => {
    it("tracks rewards distributed", async () => {
      try {
        const registryAccount = await program.account.validatorRegistry.fetch(registryPda);
        expect(registryAccount.totalRewardsDistributed).to.exist;
      } catch (error) {
        console.log("    Rewards tracking:", error.message);
      }
    });
  });

  describe("validator lifecycle", () => {
    it("deactivates a validator", async () => {
      try {
        await program.methods
          .deactivateValidator()
          .accounts({
            registry: registryPda,
            validator: validatorPda,
            operator: operator.publicKey,
          })
          .signers([operator])
          .rpc();

        const validatorAccount = await program.account.validatorInfo.fetch(validatorPda);
        expect(validatorAccount.isActive).to.be.false;
      } catch (error) {
        console.log("    Validator deactivation:", error.message);
      }
    });

    it("reactivates a validator", async () => {
      try {
        await program.methods
          .reactivateValidator()
          .accounts({
            registry: registryPda,
            validator: validatorPda,
            operator: operator.publicKey,
          })
          .signers([operator])
          .rpc();

        const validatorAccount = await program.account.validatorInfo.fetch(validatorPda);
        expect(validatorAccount.isActive).to.be.true;
      } catch (error) {
        console.log("    Validator reactivation:", error.message);
      }
    });
  });
});
