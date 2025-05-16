# Tad Contracts

**A suite of Anchor programs for the Trusted Auto Data (TAD) platform on Solana**
– Car registration, dealer & user on-chain profiles
– Odometer (km) tracking & error reporting
– Service NFT minting tied to car state
– Points & reward system

---

## 📦 Repository Structure

```
.
├── Anchor.toml
├── programs/
│   └── tad_contracts/           # Anchor Rust program
│       ├── src/
│       │   ├── instructions/     # init_car, init_config, init_dealer, init_user, register_km, report_error, register_service, add_points
│       │   ├── state/            # account definitions: Car, Config, Dealer, User, ReportData
│       │   ├── events/           # KmRegistered, ErrorReported
│       │   └── lib.rs            # program entrypoints
│       └── Cargo.toml
├── tests/
│   └── tad_contracts.ts         # mo paralle tests for all instructions
├── tsconfig.json
└── package.json                 # dev & test dependencies
```

---

## ⚙️ Prerequisites

* **Rust & Cargo**
* **Solana CLI** (v1.13+)
* **Anchor CLI** (v0.29+)
* **Node.js & Yarn** (for tests)

---

## 🚀 Local Development

### 1. Launch Local Validator

```bash
# Clean slate + load Metaplex CPIs for service-NFT minting
solana-test-validator \
  -r \
  --bpf-program metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s dump_programs/metaplex_token_metadata_program.so \
  --bpf-program CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d dump_programs/metaplex_core_program.so
```

### 2. Build & Deploy

```bash
# Build your program
anchor build

# Deploy to localnet
anchor deploy --provider.cluster localnet
```

> Your `Anchor.toml` will be automatically updated with the deployed program ID.

### 3. Run Tests

```bash
yarn install
anchor test
```

This script:

1. Starts the validator (with Metaplex CPIs)
2. Builds & deploys `tad_contracts`
3. Runs the mocha tests in `tests/tad_contracts.ts`

---

## 📝 How It Works

1. **Initialize Config**
   Creates a `Config` PDA holding your admin key.

2. **Initialize Dealer & User**
   Stores on-chain profiles under PDAs `[b"dealer", authority]` and `[b"user", authority]`.

3. **Initialize Car**
   PDA `[b"car", vin]` stores `vin`, `owner`, `dealer`, and bump.
   Tracks `total_km`, allows external backends to sign KM updates.

4. **Register KM & Report Errors**
   Cpi-free instructions to bump `car.total_km` or emit an on-chain log `ErrorReported`.

5. **Register Service**
   Mints a Metaplex NFT (`CreateV2Cpi`) to the car owner, embedding VIN, total\_km, and report\_type as on-chain attributes. Saves metadata in `ReportData` PDA.

6. **Add Points to User**
   Admin-only instruction to increment on-chain `User.points`.

---

## ⚙️ Configuration

* **Anchor.toml**
  Ensure `[program.tad_contracts]` matches your deployed ID.
* **Environment**
  If you integrate with Pinata or other IPFS, add your `.env` with any JWT / gateway settings for metadata uploads in tests.

---

## 📚 Further Reading

* [Anchor Docs](https://www.anchor-lang.com/docs)
* [Solana Cookbook](https://solanacookbook.com)
* [Metaplex Core & Token Metadata CPI](https://docs.metaplex.com)
