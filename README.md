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
  - [Instructions](#instructions)
    - [1. create\_service](#1-create_service)
      - [Accounts](#accounts)
      - [Instruction Logic](#instruction-logic)
        - [1.Initialize Service Account:](#1initialize-service-account)
        - [2.Initialize Mint and Metadata:](#2initialize-mint-and-metadata)
    - [2. buy\_service](#2-buy_service)
      - [Accounts](#accounts-1)
      - [Instruction Logic](#instruction-logic-1)
        - [1.Retrieve Service Price:](#1retrieve-service-price)
        - [2.Calculate Royalties:](#2calculate-royalties)
        - [3.Transfer Funds](#3transfer-funds)
        - [4.Update Service Ownership:](#4update-service-ownership)
        - [5.Emit Event:](#5emit-event)
    - [3. ask\_service](#3-ask_service)
      - [Accounts](#accounts-2)
      - [Instruction Logic](#instruction-logic-2)
        - [1.Initialize Ask Account:](#1initialize-ask-account)
        - [2.Send Lamports:](#2send-lamports)
        - [3.Emit Event:](#3emit-event)
    - [4. accept\_ask](#4-accept_ask)
      - [Accounts](#accounts-3)
      - [Instruction Logic](#instruction-logic-3)
        - [1. Initialize Accounts:](#1-initialize-accounts)
        - [2. Calculate and Transfer Royalties (if applicable):](#2-calculate-and-transfer-royalties-if-applicable)
        - [3. Transfer the Remaining Amount:](#3-transfer-the-remaining-amount)
        - [4. Update Service Ownership:](#4-update-service-ownership)
    - [5. withdraw\_service](#5-withdraw_service)
      - [Accounts](#accounts-4)
      - [Instruction Logic](#instruction-logic-4)
        - [1. Check Soulbound Status:](#1-check-soulbound-status)
        - [2. Transfer NFT to Vendor:](#2-transfer-nft-to-vendor)
    - [License](#license)

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

## Instructions

### 1. create_service

#### Accounts
The ListService struct validates the accounts involved in the create_service instruction. It ensures the following accounts are provided:

- **vendor**: The signer account of the vendor listing the service.
- **config_account**: The configuration account holding global settings.
- **nft_mint**: The mint account for the service NFT.
- **config_token_account**: An unchecked account that will be created for the user.
- **service_account**: The service account holding service-specific data.
- **token_program**: The SPL Token 2022 program.
- **rent**: The rent sysvar.
- **associated_token_program**: The Associated Token program.
- **system_program**: The Solana system program.

#### Instruction Logic
##### 1.Initialize Service Account:

- Set the bump, is_soulbound, current_vendor, original_vendor, nft_mint, and price fields for the service_account.

##### 2.Initialize Mint and Metadata:

- Calculate the space required for the mint account, including the metadata.
- Create the mint account and assign it to the SPL Token 2022 program.
- Initialize the metadata pointer for the mint.
- Optionally initialize the NonTransferable extension if the service is marked as soulbound.
- Initialize the mint account with the necessary settings.
- Set up the PDA for mint authority.
- Initialize the token metadata.
- Update metadata fields for each service agreement provided.
- Create the associated token account for the NFT.
- Mint one token to the associated token account.
- Freeze the mint authority to finalize the NFT.



### 2. buy_service

The buy_service instruction allows a buyer to purchase a service on the marketplace. This process involves handling the payment, including calculating and distributing royalties if the service has changed hands from the original vendor.

#### Accounts
The BuyService struct validates the accounts involved in the buy_service instruction. It ensures the following accounts are provided:

- **buyer**: The signer account of the buyer purchasing the service.
- **current_vendor**: The current vendor who will receive the lamports. This is an unchecked account used to send lamports.
- **original_vendor**: The original vendor who may receive royalty payments. This is an unchecked account used to send lamports.
- **config_account**: The configuration account holding global settings, such as the royalty fee basis points.
- **service_account**: The service account holding service-specific data. This account is validated to ensure it matches the expected vendors.
- **rent**: The rent sysvar.
- **system_program**: The Solana system program.

#### Instruction Logic

##### 1.Retrieve Service Price:

- The service price is fetched from the service_account.

##### 2.Calculate Royalties:

- If the current vendor is different from the original vendor, the royalty amount is calculated based on the royalty fee basis points from the config_account.

##### 3.Transfer Funds

- Royalties are sent to the original vendor if applicable.
- The remaining amount is sent to the current vendor.

##### 4.Update Service Ownership:

- The current_vendor in the service_account is updated to the buyer.

##### 5.Emit Event:

- A BuyServiceEvent is emitted with details of the transaction.

### 3. ask_service

The ask_service instruction allows a user to place a request (or "ask") for a service on the marketplace. This involves setting a price they are willing to pay and creating an AskAccount that holds this information.

#### Accounts

The AskService struct validates the accounts involved in the ask_service instruction. It ensures the following accounts are provided:

- **asker**: The signer account of the user placing the ask.
- **config_account**: The configuration account holding global settings.
- **nft_mint**: The mint account associated with the service NFT. This is an unchecked account.
- **ask_account**: The account that will store the ask details. It will be created if it does not exist.
- **rent**: The rent sysvar.
- **system_program**: The Solana system program.

#### Instruction Logic

##### 1.Initialize Ask Account:

- Set the bump, ask_price, asker, and nft_mint fields for the ask_account.

##### 2.Send Lamports:

- Transfer the ask_price amount of lamports from the asker to the ask_account.

##### 3.Emit Event:

- Emit an AskServiceEvent with details of the ask.

### 4. accept_ask

The accept_ask instruction allows a vendor to accept a user's request (or "ask") for a service on the marketplace. This involves transferring the agreed price, calculating and handling royalties if necessary, and updating the service ownership.

#### Accounts

The AccpectAsk struct validates the accounts involved in the accept_ask instruction. It ensures the following accounts are provided:

- **vendor**: The signer account of the vendor accepting the ask.
- **asker**: The account of the user who placed the ask. This is an unchecked account.
- **config_account**: The configuration account holding global settings.
- **ask_account**: The account storing the ask details.
- **service_account**: The service account holding service-specific data.
- **rent**: The rent sysvar.
- **system_program**: The Solana system program.

#### Instruction Logic

##### 1. Initialize Accounts:

- Retrieve and set necessary fields from ask_account, config_account, and service_account.
##### 2. Calculate and Transfer Royalties (if applicable):

- If the vendor is different from the original vendor, calculate the royalty fee.
- Transfer the royalty amount to the original vendor.
- Transfer the remaining amount to the current vendor.
##### 3. Transfer the Remaining Amount:
- If no royalties are due, transfer the entire ask price to the current vendor.

##### 4. Update Service Ownership:
- Update the current_vendor field in the service_account to the asker.


### 5. withdraw_service

The withdraw_service instruction allows a vendor to withdraw a non-soulbound NFT from the service pool to their personal account. This involves transferring the NFT from the pool to the vendor's associated token account.

#### Accounts

The WithdrawNFTService struct validates the accounts involved in the withdraw_service instruction. It ensures the following accounts are provided:

- **vendor**: The signer account of the vendor withdrawing the NFT.
- **config_account**: The configuration account holding global settings.
- **config_token_account**: The token account associated with the configuration.
- **nft_mint**: The mint account of the NFT.
- **vendor_token_account**: The vendor's associated token account for the NFT. It will be created if it does not exist.
- **service_account**: The service account holding service-specific data.
- **token program**: The SPL Token 2022 program.
- **associated_token_program**: The Associated Token program.
- **rent**: The rent sysvar.
- **system_program**: The Solana system program.


#### Instruction Logic

##### 1. Check Soulbound Status:

-If the NFT is marked as soulbound, return an error since soulbound NFTs are non-transferable.

##### 2. Transfer NFT to Vendor:

- Use the helper function transfer_nft_from_pool_to_user to transfer the NFT from the pool's token account to the vendor's token account.

### License
```
This project is licensed under the MIT License. See the LICENSE file for details.
```