# Solana Service Marketplace Smart Contract

This repository contains a Solana program written using the Anchor framework. The program allows vendors to list services on a decentralized marketplace. The services are represented as NFTs with associated metadata, and can be marked as soulbound if required.

## Table of Contents

- [Solana Service Marketplace Smart Contract](#solana-service-marketplace-smart-contract)
  - [Table of Contents](#table-of-contents)
  - [Prerequisites](#prerequisites)
  - [Getting Started](#getting-started)
    - [1. Clone the Repository](#1-clone-the-repository)
    - [2. Install Dependencies](#2-install-dependencies)
    - [3. Build the Program](#3-build-the-program)
    - [4. Deploy the Program](#4-deploy-the-program)
    - [5. Run the Tests](#5-run-the-tests)

## Prerequisites

- Rust: [Install Rust](https://www.rust-lang.org/tools/install)
- Solana CLI: [Install Solana CLI](https://docs.solana.com/cli/install-solana-cli-tools)
- Anchor CLI: [Install Anchor CLI](https://book.anchor-lang.com/chapter_2/0.1.0/install.html)

## Getting Started

### 1. Clone the Repository

```sh
git clone https://github.com/your-repo/solana-service-marketplace.git
cd solana-service-marketplace
```
### 2. Install Dependencies

```sh
cargo install --locked --force solana-cli
cargo install --locked --force anchor-cli
```

### 3. Build the Program
```sh
anchor build
```

### 4. Deploy the Program
```
anchor deploy
```

### 5. Run the Tests
```
anchor test
```