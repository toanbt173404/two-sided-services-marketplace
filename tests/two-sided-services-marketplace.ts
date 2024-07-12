import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";

import { TwoSidedServicesMarketplace } from "../target/types/two_sided_services_marketplace";
import { Keypair, PublicKey, Connection, Signer, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_2022_PROGRAM_ID, getAssociatedTokenAddressSync } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

describe("two-sided-services-marketplace", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.TwoSidedServicesMarketplace as Program<TwoSidedServicesMarketplace>;
  const connection = new Connection("http://localhost:8899", 'confirmed');

  let admin: Signer;
  let vendor: Signer;
  let buyer: Signer;
  let asker: Signer;

  let serviceAccount: PublicKey;
  let askAccount: PublicKey;
  let configTokenAccount: PublicKey;
  let nftMint: Keypair;

  const royaltyFeeBasisPoints = 100 //~1%;

  const price = new anchor.BN(0.1 * LAMPORTS_PER_SOL) // 0.1 SOL;

  const askPrice = new anchor.BN(0.5 * LAMPORTS_PER_SOL) //0.5 SOl

  const configAccount = PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  )[0];

  it("Is initialized!", async () => {
    admin = await createUserWithLamports(connection, 10);
    await program.methods.initialize(royaltyFeeBasisPoints).accountsPartial({
      admin: admin.publicKey
    }).signers([admin]).rpc();

    const configData = await program.account.configAccount.fetch(configAccount);
    expect(configData.isInitialized).to.equal(true);
    expect(configData.admin.toString()).to.equal(admin.publicKey.toString());
    expect(configData.royaltyFeeBasisPoints).to.equal(royaltyFeeBasisPoints);
  });

  it("Create services", async () => {

    buyer = await createUserWithLamports(connection, 10);
    nftMint = Keypair.generate();

    serviceAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("service"), nftMint.publicKey.toBuffer()],
      program.programId
    )[0];

    vendor = await createUserWithLamports(connection, 10);

    configTokenAccount = getAssociatedTokenAddressSync(
      nftMint.publicKey,
      configAccount,
      true,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    const isSouldBound = false;

    const agreements = [
      { title: "agreement1", details: "details1" },
      { title: "agreement2", details: "details2" },
    ];

    await program.methods.createService(isSouldBound, agreements, price).accountsPartial({
      vendor: vendor.publicKey,
      configAccount: configAccount,
      nftMint: nftMint.publicKey,
      serviceAccount: serviceAccount,
      configTokenAccount: configTokenAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([vendor, nftMint]).rpc();

    const serviceData = await program.account.serviceAccount.fetch(serviceAccount);
    expect(serviceData.isSoulbound).to.equal(isSouldBound);
    expect(serviceData.originalVendor.toString()).to.equal(vendor.publicKey.toString());
    expect(serviceData.currentVendor.toString()).to.equal(vendor.publicKey.toString());
    expect(serviceData.price.toString()).to.equal(price.toString());
  })

  it("Buy service", async () => {
    await program.methods.buyService().accountsPartial({
      buyer: buyer.publicKey,
      configAccount: configAccount,
      serviceAccount: serviceAccount,
      currentVendor: vendor.publicKey,
      originalVendor: vendor.publicKey,
    }).signers([buyer]).rpc();

    const serviceData = await program.account.serviceAccount.fetch(serviceAccount);

    expect(serviceData.currentVendor.toString()).to.equal(buyer.publicKey.toString());
  })

  it("Update new service price", async () => {
    const newServicePrice = new anchor.BN(0.7 * LAMPORTS_PER_SOL);
    await program.methods.updateServicePrice(newServicePrice).accountsPartial({
      vendor: buyer.publicKey,
      serviceAccount: serviceAccount,
    }).signers([buyer]).rpc();

    const serviceData = await program.account.serviceAccount.fetch(serviceAccount);

    expect(serviceData.price.toString()).to.equal(newServicePrice.toString());
  })

  it("Ask service", async () => {
    asker = await createUserWithLamports(connection, 10);

    askAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("ask"), nftMint.publicKey.toBuffer()],
      program.programId
    )[0];

    await program.methods.askService(askPrice).accountsPartial({
      asker: asker.publicKey,
      configAccount: configAccount,
      askAccount: askAccount,
      nftMint: nftMint.publicKey
    }).signers([asker]).rpc();

    const askData = await program.account.askAccount.fetch(askAccount);
    expect(askData.asker.toString()).to.equal(asker.publicKey.toString());
    expect(askData.nftMint.toString()).to.equal(nftMint.publicKey.toString());
    expect(askData.askPrice.toString()).to.equal(askPrice.toString());
  })

  it("Update new ask price", async () => {
    const newAskPrice = new anchor.BN(0.8 * LAMPORTS_PER_SOL);
    await program.methods.updateAskPrice(newAskPrice).accountsPartial({
      asker: asker.publicKey,
      askAccount: askAccount,
    }).signers([asker]).rpc();

    const askData = await program.account.askAccount.fetch(askAccount);

    expect(askData.askPrice.toString()).to.equal(newAskPrice.toString());
  })

  it("Accpect ask", async () => {
    await program.methods.acceptAsk().accountsPartial({
      vendor: buyer.publicKey,
      asker: asker.publicKey,
      configAccount: configAccount,
      askAccount: askAccount,
      serviceAccount: serviceAccount,
      originalVendor: vendor.publicKey
    }).signers([buyer]).rpc();

    const serviceData = await program.account.serviceAccount.fetch(serviceAccount);

    expect(serviceData.currentVendor.toString()).to.equal(asker.publicKey.toString())
  });

  it("Withdraw service", async () => {

    const vendorTokenAccount = getAssociatedTokenAddressSync(
      nftMint.publicKey,
      asker.publicKey,
      false,
      TOKEN_2022_PROGRAM_ID,
      ASSOCIATED_TOKEN_PROGRAM_ID
    );

    await program.methods.withdrawService().accountsPartial({
      vendor: asker.publicKey,
      configAccount: configAccount,
      serviceAccount: serviceAccount,
      nftMint: nftMint.publicKey,
      configTokenAccount: configTokenAccount,
      vendorTokenAccount: vendorTokenAccount,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([asker]).rpc();
  })
});

export async function createUserWithLamports(
  connection: Connection,
  lamports: number,
): Promise<Signer> {
  const account = Keypair.generate();
  const signature = await connection.requestAirdrop(
    account.publicKey,
    lamports * LAMPORTS_PER_SOL
  );
  const block = await connection.getLatestBlockhash();
  await connection.confirmTransaction({ ...block, signature });
  return account;
}