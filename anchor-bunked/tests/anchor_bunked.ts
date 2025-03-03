import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorBunked } from "../target/types/anchor_bunked";
import { PublicKey } from "@solana/web3.js";
import { expect, assert } from "chai";
import * as fs from "fs";
import * as crypto from "crypto";
import * as nacl from "tweetnacl";

describe("anchor_bunked", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.AnchorBunked as Program<AnchorBunked>;
  const wallet = anchor.getProvider().wallet as anchor.Wallet;

  const creator = anchor.web3.Keypair.generate();
  const manifestA = String("Qmdb9o1G6xYJUZ3Dur21pb6Z9HdrRcXRaWnmJ8VRENB1DP");
  const manifestB = String("QmdxB27SFrkEXwn7xnoSLfkp29bjLP2acsxmnZnBpCU839");
  const phashA = new anchor.BN("18421074668219899903");
  // const phashA = new anchor.BN("42");
  const phashB = new anchor.BN("18421074668219899904");

  // Given a photo sha256 calc
  const imageBuffer = fs.readFileSync(
    "/home/sam/photo_2025-02-21_13-15-10.jpg"
  );
  const imageHash = crypto.createHash("sha256").update(imageBuffer).digest();

  const signature = nacl.sign.detached(imageHash, creator.secretKey);

  function getIndexPDA(phash: anchor.BN): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("index"), phash.toBuffer("le", 8)],
      // [Buffer.from("index"), phash.toArrayLike(Buffer,"le",8)],
      program.programId
    );
  }

  function getManifestPDA(
    phash: anchor.BN,
    index: number
  ): [PublicKey, number] {
    return PublicKey.findProgramAddressSync(
      [
        Buffer.from("manifest"),
        phash.toBuffer("le", 8),
        new anchor.BN(index).toBuffer("le", 8),
      ],
      program.programId
    );
  }

  // Utility to get the current index or 0 if account does not exist
  async function getIndex(phash: anchor.BN): Promise<number> {
    const [indexPDA] = getIndexPDA(phash);

    try {
      const indexAccount = await program.account.indexAccount.fetch(indexPDA);
      return indexAccount.index.toNumber();
    } catch {
      // Index account does not exist yet, so return 0
      return 0;
    }
  }

  it("Register two new Manifests, ensure index starts at 0 for both", async () => {
    const [indexPDA_A] = getIndexPDA(phashA);
    const indexA = await getIndex(phashA);
    const [manifestPDA_A] = getManifestPDA(phashA, indexA);

    const tx1 = await program.methods
      .register(phashA, manifestA, Array.from(imageHash), Array.from(signature), null)
      // .register(phashA, manifestA, Array.from(imageHash), Array.from(signature))
      .accounts({
        creator: wallet.publicKey,
        indexAccount: indexPDA_A,
        manifestAccount: manifestPDA_A,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const indexAccountA = await program.account.indexAccount.fetch(indexPDA_A);
    const manifestAccountA = await program.account.manifestAccount.fetch(
      manifestPDA_A
    );

    expect(indexAccountA.phash.toString()).to.equal(phashA.toString());
    expect(indexAccountA.index.toNumber()).to.equal(indexA + 1);
    expect(manifestAccountA.index.toNumber()).to.equal(indexA);
    expect(manifestAccountA.cid).to.equal(manifestA);

    // Create a second account to ensure its seed worked differently
    const [indexPDA_B] = getIndexPDA(phashB);
    const indexB = await getIndex(phashB);
    const [manifestPDA_B] = getManifestPDA(phashB, indexB);

    const tx2 = await program.methods
      .register(phashB, manifestB, Array.from(imageHash), Array.from(signature), null)
      // .register(phashB, manifestB,Array.from(imageHash), Array.from(signature))
      .accounts({
        creator: wallet.publicKey,
        indexAccount: indexPDA_B,
        manifestAccount: manifestPDA_B,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const indexAccountB = await program.account.indexAccount.fetch(indexPDA_B);
    const manifestAccountB = await program.account.manifestAccount.fetch(
      manifestPDA_B
    );

    expect(indexAccountB.phash.toString()).to.equal(phashB.toString());
    expect(indexAccountB.index.toNumber()).to.equal(indexB+1);
    expect(manifestAccountB.index.toNumber()).to.equal(indexB);
    expect(manifestAccountB.cid).to.equal(manifestB);

    console.log("Your transaction signature", tx1);
    console.log("Your transaction signature", tx2);
  });

  it("Register a second manifest with matching phash, ensure index is increased", async () => {
    const [indexPDA_A] = getIndexPDA(phashA);
    const indexA = await getIndex(phashA);
    const [manifestPDA_A] = getManifestPDA(phashA, indexA);

    const tx = await program.methods
      .register(phashA, manifestA, Array.from(imageHash), Array.from(signature), null)
      .accounts({
        creator: wallet.publicKey,
        indexAccount: indexPDA_A,
        manifestAccount: manifestPDA_A,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const indexAccount = await program.account.indexAccount.fetch(indexPDA_A);
    const manifestAccount = await program.account.manifestAccount.fetch(
      manifestPDA_A
    );

    expect(indexAccount.phash.toString()).to.equal(phashA.toString());
    expect(indexAccount.index.toNumber()).to.equal(indexA + 1);
    expect(manifestAccount.index.toNumber()).to.equal(indexA);
    expect(manifestAccount.cid).to.equal(manifestA);

    console.log("Your transaction signature", tx);
  });

  it("Ensure failure if CID is invalid", async () => {
    const [indexPDA_A] = getIndexPDA(phashA);
    const indexA = await getIndex(phashA);
    const [manifestPDA_A] = getManifestPDA(phashA, indexA);

    const manifest_too_long = String(
      "Qmdb9o1G6xYJUZ3Dur21pb6Z9HdrRcXRaWnmJ8VRENB1DPA"
    );

    const manifest_too_short = String(
      "Qmdb9o1G6xYJUZ3Dur21pb6Z9HdrRcXRaWnmJ8VRENB"
    );

    try {
      const tx = await program.methods
        .register(
          phashA,
          manifest_too_long,
          Array.from(imageHash),
          Array.from(signature),
          null
        )
        .accounts({
          creator: wallet.publicKey,
          indexAccount: indexPDA_A,
          manifestAccount: manifestPDA_A,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("Expected n error, but the transaction succeeded");
    } catch (err) {
      assert.include(err.message, "CID is invalid");
    }

    try {
      const tx = await program.methods
        .register(
          phashA,
          manifest_too_short,
          Array.from(imageHash),
          Array.from(signature),
          null,
        )
        .accounts({
          creator: wallet.publicKey,
          indexAccount: indexPDA_A,
          manifestAccount: manifestPDA_A,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();

      assert.fail("Expected n error, but the transaction succeeded");
    } catch (err) {
      assert.include(err.message, "CID is invalid");
    }
  });

  it("Ensure it works with or without Mint Account", async () => {

    const mintA = anchor.web3.Keypair.generate();

    const [indexPDA_A] = getIndexPDA(phashA);
    const indexA = await getIndex(phashA);
    const [manifestPDA_A] = getManifestPDA(phashA, indexA);

    const tx1 = await program.methods
      .register(phashA, manifestA, Array.from(imageHash), Array.from(signature), mintA.publicKey)
      .accounts({
        creator: wallet.publicKey,
        indexAccount: indexPDA_A,
        manifestAccount: manifestPDA_A,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const manifestAccount = await program.account.manifestAccount.fetch(
      manifestPDA_A
    );
    expect(new anchor.web3.PublicKey(manifestAccount.mint).toBase58()).to.equal(mintA.publicKey.toBase58());
    console.log("Your transaction signature", tx1);

    // Create a second account to ensure its seed worked differently
    const [indexPDA_B] = getIndexPDA(phashB);
    const indexB = await getIndex(phashB);
    const [manifestPDA_B] = getManifestPDA(phashB, indexB);

    const tx2 = await program.methods
      .register(phashB, manifestB, Array.from(imageHash), Array.from(signature), null)
      // .register(phashB, manifestB,Array.from(imageHash), Array.from(signature))
      .accounts({
        creator: wallet.publicKey,
        indexAccount: indexPDA_B,
        manifestAccount: manifestPDA_B,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const manifestAccountB = await program.account.manifestAccount.fetch(
      manifestPDA_B
    );
    expect(manifestAccountB.mint).to.be.null;
    console.log("Your transaction signature", tx2);
  });

  it("Attempt searching all images with a give pHash", async () => {
    const [indexPDA_A] = getIndexPDA(phashA);
    const indexA = await getIndex(phashA);

    let storedCIDs: String[] = [];

    for (let i=0; i < indexA; i++) {
      const [manifestPDA] = getManifestPDA(phashA, i);
      const manifestAccount = await program.account.manifestAccount.fetch(manifestPDA);
      storedCIDs.push(manifestAccount.cid.toString())
      expect(manifestAccount.cid.toString()).to.equal(manifestA);
    }
    console.log("Here are the registered and signed images with matching pHash", storedCIDs)
  })

  it("Attempt searching a non existing pHash", async () => {
    const phash_temp = new anchor.BN("18421074668219899900");
    const [indexPDA_temp] = getIndexPDA(phash_temp);
    const index_temp = await getIndex(phash_temp);

    expect(index_temp).to.equal(0);
  })

  it("Attempt searching out of bounds", async () => {
    const phash_temp = new anchor.BN("18421074668219899900");
    const [indexPDA_temp] = getIndexPDA(phash_temp);
    const index_temp = await getIndex(phash_temp);

    const [manifestPDA] = getManifestPDA(phashA, index_temp + 1);
    // const manifestAccount = await program.account.manifestAccount.fetch(manifestPDA);
    // storedCIDs.push(manifestAccount.cid.toString())
    // expect(manifestAccount.cid.toString()).to.equal(manifestA);

    // expect(index_temp).to.equal(0);
  })
});
