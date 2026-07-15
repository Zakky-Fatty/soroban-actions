# soroban-actions

<div align="center">

![GitHub Actions](https://img.shields.io/badge/GitHub%20Actions-composite%20action-2088FF?logo=github-actions&logoColor=white)
![Stellar](https://img.shields.io/badge/Stellar-Soroban-7B2FBE?logo=stellar&logoColor=white)
![Rust](https://img.shields.io/badge/Rust-stable-CE4B27?logo=rust&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-green.svg)
![Stellar Wave](https://img.shields.io/badge/Stellar-Wave%20Program-0070F3?logo=stellar)

**A production-grade composite GitHub Action that automates the full Soroban smart-contract lifecycle**  
*Install → Build → Optimise → Test → Deploy*

</div>

---

## ✨ Features

| Step | What it does |
|------|-------------|
| 🦀 **Rust toolchain** | Pins any Rust toolchain version, adds `wasm32-unknown-unknown` target automatically |
| 📦 **Cargo cache** | Caches registry + build artefacts keyed by `Cargo.lock` hash for fast incremental builds |
| 🌟 **Stellar CLI** | Installs a specific or `latest` version of `stellar-cli` via Cargo |
| ⚙️ **wasm-opt** | Installs `wasm-opt` (binaryen) and runs `Os` optimisation + debug-symbol stripping |
| 🔨 **Build** | `cargo build --target wasm32-unknown-unknown --release` |
| 🧪 **Test** | `cargo test --workspace --all-features` with `RUST_BACKTRACE=1` |
| 🚀 **Deploy (simulate)** | `stellar contract deploy --simulate-only` against any Stellar network |

---

## 🚀 Quick Start

### Minimal – Build & Test only

```yaml
# .github/workflows/ci.yml
name: Soroban CI

on: [push, pull_request]

jobs:
  ci:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Build & test Soroban contract
        uses: Alhaji-naira/soroban-actions@v1
        with:
          contract-path: .        # path to Cargo.toml directory
          run-build: "true"
          run-tests: "true"
```

### Full pipeline – Build, Test, and Simulate Testnet Deploy

```yaml
name: Soroban Full Pipeline

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Run full Soroban pipeline
        id: soroban
        uses: Alhaji-naira/soroban-actions@v1
        with:
          stellar-cli-version: "21.3.1"   # pin a specific version
          rust-toolchain: "stable"
          contract-path: contracts/my-contract
          run-build:  "true"
          run-tests:  "true"
          run-deploy: "true"
          deploy-source-account: ${{ secrets.STELLAR_SOURCE_ACCOUNT }}
          deploy-network: testnet
          deploy-rpc-url: "https://soroban-testnet.stellar.org"

      - name: Print outputs
        run: |
          echo "Wasm path : ${{ steps.soroban.outputs.wasm-path }}"
          echo "Sim TX ID : ${{ steps.soroban.outputs.deploy-tx-id }}"
```

### Monorepo – Multiple contracts in one workflow

```yaml
name: Monorepo Contract CI

on: [push]

jobs:
  contracts:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        contract: [token, nft, amm]

    steps:
      - uses: actions/checkout@v4

      - name: "CI for ${{ matrix.contract }}"
        uses: Alhaji-naira/soroban-actions@v1
        with:
          contract-path: contracts/${{ matrix.contract }}
          run-build: "true"
          run-tests: "true"
```

---

## 📋 Inputs Reference

| Input | Required | Default | Description |
|-------|----------|---------|-------------|
| `stellar-cli-version` | No | `latest` | Stellar CLI version to install. Set to a SemVer string (e.g. `21.3.1`) or `latest`. |
| `rust-toolchain` | No | `stable` | Rust toolchain channel: `stable`, `nightly`, or a specific version like `1.79`. |
| `contract-path` | No | `.` | Path to the crate/workspace root containing `Cargo.toml`. |
| `run-build` | No | `true` | Compile the contract to wasm32 and run `wasm-opt`. |
| `run-tests` | No | `true` | Execute `cargo test --workspace --all-features`. |
| `run-deploy` | No | `false` | Simulate a deployment to Stellar Testnet via `stellar contract deploy --simulate-only`. |
| `deploy-network` | No | `testnet` | Network alias (e.g. `testnet`, `mainnet`). |
| `deploy-source-account` | No | `""` | Source account alias or secret key for deployment. Store as a GitHub Secret. |
| `deploy-network-passphrase` | No | `Test SDF Network ; September 2015` | Network passphrase. |
| `deploy-rpc-url` | No | `https://soroban-testnet.stellar.org` | Soroban RPC endpoint. |
| `cache-cargo` | No | `true` | Enable Cargo registry + build-artefact caching. |

---

## 📤 Outputs Reference

| Output | Description |
|--------|-------------|
| `wasm-path` | Relative path to the optimised `*-opt.wasm` binary produced by the build step. |
| `deploy-tx-id` | Simulation transaction ID returned by the Testnet deploy step (empty if `run-deploy` is `false`). |

---

## 🔐 Secrets

| Secret | Required | Description |
|--------|----------|-------------|
| `STELLAR_SOURCE_ACCOUNT` | Only when `run-deploy: "true"` | Stellar account secret key (`S...`) or a pre-configured account alias. **Never commit raw keys.** |

---

## 🏗️ Architecture

```
soroban-actions/
├── action.yml                  ← Composite action definition (entry point)
├── dummy-contract/             ← Dogfood contract used by CI to self-validate
│   ├── Cargo.toml
│   └── src/lib.rs              ← CounterContract + 6 unit tests
└── .github/
    └── workflows/
        └── test.yml            ← CI that exercises this very action
```

### Composite action flow

```
┌─────────────────────────────────────────────────────────────────┐
│                       soroban-actions                           │
├─────────────────────────────────────────────────────────────────┤
│  1. Install Rust toolchain + wasm32-unknown-unknown target       │
│  2. Restore Cargo cache (keyed by Cargo.lock hash)              │
│  3. Install stellar-cli (pinned or latest)                      │
│  4. Install wasm-opt  [if run-build]                            │
│  5. cargo build --target wasm32-unknown-unknown --release        │
│     [if run-build]                                              │
│  6. wasm-opt -Os --strip-debug  [if run-build]                  │
│  7. cargo test --workspace --all-features  [if run-tests]       │
│  8. stellar contract deploy --simulate-only  [if run-deploy]    │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🧪 Dogfood CI Jobs

The bundled `test.yml` runs five jobs against every push / PR:

| Job | Purpose |
|-----|---------|
| `lint` | `yamllint` + `actionlint` validate all YAML files |
| `build` | Matrix: ubuntu-latest + macos-latest – build-only |
| `test` | ubuntu-latest – build + test |
| `simulate-deploy` | Runs on `main` push / `workflow_dispatch` – full pipeline with simulate |
| `ci-gate` | Required status check that blocks merges if any job fails |

---

## 🛠️ Local Development

```bash
# Clone and enter the repo
git clone https://github.com/Alhaji-naira/soroban-actions.git
cd soroban-actions

# Install the Rust wasm target
rustup target add wasm32-unknown-unknown

# Build the dummy contract locally
cd dummy-contract
cargo build --target wasm32-unknown-unknown --release

# Run tests locally
cargo test -- --nocapture

# Optimise manually (requires wasm-opt)
wasm-opt -Os --strip-debug \
  target/wasm32-unknown-unknown/release/dummy_contract.wasm \
  -o target/wasm32-unknown-unknown/release/dummy_contract-opt.wasm
```

---

## 📌 Versioning

This action follows [Semantic Versioning](https://semver.org/).

```yaml
# Always use the latest stable release
uses: Alhaji-naira/soroban-actions@v1

# Pin to a specific patch for maximum reproducibility
uses: Alhaji-naira/soroban-actions@v1.0.0
```

> **Tip:** Point to a commit SHA for security-sensitive supply chains:
> `uses: Alhaji-naira/soroban-actions@<sha>`

---

## 🤝 Contributing

1. Fork → feature branch → PR against `main`.
2. All PRs must pass the dogfood CI gate.
3. Update `CHANGELOG.md` with a brief entry.
4. Adhere to the existing YAML style (2-space indent, no trailing whitespace).

---

## 📄 License

[MIT](LICENSE) © 2026 Alhaji-naira  
Built for the **Stellar Wave Program**.
