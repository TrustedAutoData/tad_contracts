import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TadContracts } from "../target/types/tad_contracts";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";

describe("tad_contracts", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.TadContracts as Program<TadContracts>;
  const provider = anchor.AnchorProvider.env();
  const admin = provider.wallet as anchor.Wallet;

  let configPda: PublicKey;
  let dealerPda: PublicKey;
  let userPda: PublicKey;
  let carPda: PublicKey;

  const vin = "VIN-001-XYZ";
  const dealerName = "TestDealer";
  const userEmail = "user@example.com";

  // Create a new keypair to act as the car owner
  const owner = anchor.web3.Keypair.generate();

  before(async () => {
    // Airdrop SOL to the owner
    const airdropSig = await provider.connection.requestAirdrop(
      owner.publicKey,
      1e9
    ); // 1 SOL
    await provider.connection.confirmTransaction(airdropSig);

    [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    [dealerPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("dealer"), admin.publicKey.toBuffer()],
      program.programId
    );

    [userPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("user"), admin.publicKey.toBuffer()],
      program.programId
    );

    [carPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("car"), Buffer.from(vin)],
      program.programId
    );
  });

  it("Initializes config", async () => {
    const tx = await program.methods
      .initializeConfig()
      .accounts({
        config: configPda,
        admin: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Config initialized:", tx);

    const config = await program.account.config.fetch(configPda);
    assert.ok(config.admin.equals(admin.publicKey));
  });

  it("Initializes dealer", async () => {
    const tx = await program.methods
      .initializeDealer(dealerName)
      .accounts({
        dealer: dealerPda,
        authority: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ Dealer initialized:", tx);

    const dealer = await program.account.dealer.fetch(dealerPda);
    assert.equal(dealer.name, dealerName);
  });

  it("Initializes user", async () => {
    const tx = await program.methods
      .initializeUser(userEmail)
      .accounts({
        user: userPda,
        authority: admin.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    console.log("✅ User initialized:", tx);

    const user = await program.account.user.fetch(userPda);
    assert.equal(user.email, userEmail);
  });

  it("Initializes car", async () => {
    // Derive the car PDA using the VIN
    const [carPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("car"), Buffer.from(vin)],
      program.programId
    );

    const tx = await program.methods
      .initializeCar(vin)
      .accounts({
        car: carPda,
        dealer: dealerPda,
        owner: owner.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([owner])
      .rpc();

    console.log("✅ Car initialized:", tx);

    const car = await program.account.car.fetch(carPda);
    assert.equal(car.vin, vin);
    assert.ok(car.owner.equals(owner.publicKey));
  });

  it("Registers car KM", async () => {
    const km = 5000;

    const tx = await program.methods
      .registerCarKm(new anchor.BN(km))
      .accounts({
        car: carPda,
        owner: owner.publicKey,
      })
      .rpc();

    console.log("✅ KM registration tx:", tx);

    const car = await program.account.car.fetch(carPda);
    console.log("🚗 Total KM after update:", car.totalKm.toString());

    assert.equal(car.totalKm.toNumber(), km);
  });

  it("Reports car error", async () => {
    const errorCode = 101;
    const errorMessage = "Engine Overheating";

    const tx = await program.methods
      .reportCarError(errorCode, errorMessage)
      .accounts({
        car: carPda,
      })
      .rpc();

    console.log("🚨 Error report tx:", tx);
    console.log("🛠️ Reported error:", {
      vin,
      code: errorCode,
      message: errorMessage,
    });

    const car = await program.account.car.fetch(carPda);
    console.log("📋 VIN confirmed:", car.vin);
    assert.ok(car.vin === vin);
  });
});
