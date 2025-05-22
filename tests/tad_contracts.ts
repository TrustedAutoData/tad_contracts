import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TadContracts } from "../target/types/tad_contracts";
import { PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import { PinataSDK } from "pinata-web3";
import * as dotenv from "dotenv";
import { metadata } from "../metadata/nft.tad";

dotenv.config();
const pinata = new PinataSDK({
  pinataJwt: process.env.PINATA_JWT || "",
  pinataGateway: process.env.PINATA_GATEWAY || "",
});

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

  const creator = provider.wallet as anchor.Wallet;
  const reportId = new anchor.BN(1);
  const reportName = "Inspection Pass";
  const contentUri = "https://example.com/metadata.json";
  const organizationName = "ServicePro";

  let reportDataPda: PublicKey;
  let dealerReportDataPda: PublicKey;

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

    [carPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("car"), Buffer.from(vin)],
      program.programId
    );

    [reportDataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("report_data"),
        carPda.toBuffer(),
        reportId.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    [dealerReportDataPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("dealer_report_data"),
        carPda.toBuffer(),
        reportId.toArrayLike(Buffer, "le", 8),
      ],
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

  it("Registers service and mints NFT", async () => {
    const metadataUpload = await pinata.upload.json(metadata);
    const uri = `https://${metadataUpload.IpfsHash}.ipfs.dweb.link/`;

    const ownerNft = Keypair.generate();

    const tx = await program.methods
      .registerServiceAttendance(reportId, uri, "Oil change")
      .accounts({
        car: carPda,
        reportData: reportDataPda,
        ownerNft: ownerNft.publicKey,
        creator: creator.publicKey,
        owner: owner.publicKey,
        mplTokenMetadataProgram: new PublicKey(
          "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        ),
        systemProgram: SystemProgram.programId,
        mplCoreProgram: new PublicKey(
          "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        ),
      })
      .signers([ownerNft, creator.payer])
      .rpc();

    console.log("✅ Service NFT minted:", tx);

    const report = await program.account.reportData.fetch(reportDataPda);
    // assert.equal(report.reportId.toNumber(), reportId.toNumber());
    // assert.equal(report.contentUri, contentUri);
    // assert.isTrue(report.isOwnerNft);
  });

  it("Gets car report and mints NFT", async () => {
    const reportId = new anchor.BN(1);
    const metadataUpload = await pinata.upload.json(metadata);
    const contentUri = `https://${metadataUpload.IpfsHash}.ipfs.dweb.link/`;
    const reportType = "Full Diagnostic";

    const user = anchor.web3.Keypair.generate();

    // Airdrop SOL to user
    await provider.connection.requestAirdrop(user.publicKey, 1e9);
    await new Promise((resolve) => setTimeout(resolve, 1000));

    const [carPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("car"), Buffer.from(vin)],
      program.programId
    );

    const [configPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("config")],
      program.programId
    );

    const ownerNft = anchor.web3.Keypair.generate();

    const tx = await program.methods
      .getReport(reportId, contentUri, reportType)
      .accounts({
        car: carPda,
        dealerReportData: dealerReportDataPda,
        config: configPda,
        ownerNft: ownerNft.publicKey,
        creator: admin.publicKey,
        user: user.publicKey,
        vault: new PublicKey("3Qc5TBFKHSbEKxD81e5Pcczdg8Y5goFHga6HJdKgRis5"),
        mplTokenMetadataProgram: new PublicKey(
          "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
        ),
        mplCoreProgram: new PublicKey(
          "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        ),
        systemProgram: SystemProgram.programId,
      })
      .signers([user, ownerNft])
      .rpc();

    console.log("✅ Car report minted with NFT:", tx);

    const report = await program.account.dealerReportData.fetch(
      dealerReportDataPda
    );
    assert.equal(report.reportId.toNumber(), reportId.toNumber());
    assert.equal(report.contentUri, contentUri);
  });

  it("Adds points to the user account", async () => {
    const pointsToAdd = new anchor.BN(250);

    const tx = await program.methods
      .addUserPoints(pointsToAdd)
      .accounts({
        config: configPda,
        user: userPda,
        admin: admin.publicKey,
      })
      .signers([admin.payer])
      .rpc();

    console.log("✅ Points added tx:", tx);

    const user = await program.account.user.fetch(userPda);
    console.log("🏅 User points:", user.points.toString());

    assert.equal(user.points.toString(), "250");
  });
});
