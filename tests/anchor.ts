import BN from "bn.js";
import assert from "assert";
import * as web3 from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, Keypair } from "@solana/web3.js";
import { PortfolioProgram } from "../target/types/portfolio_program";
import assert from "assert";
import type { PortfolioProgram } from "../target/types/portfolio_program";

describe("portfolio-program", () => {
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.PortfolioProgram as anchor.Program<PortfolioProgram>;
  
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace
    .PortfolioProgram as Program<PortfolioProgram>;

  const authority = program.provider.wallet.payer;
  const tipAmount = new anchor.BN(100);
  const links = ["https://myportfolio.com", "https://github.com"];
  const bio = "This is my portfolio bio.";
  const imageUrl = "https://example.com";
  const vouchUser = Keypair.generate().publicKey;
  const vouchComment = "Great portfolio!";
  const messageContent = "Hello";
  let pda: PublicKey;

  it("Is initialized!", async () => {
    const [pda1, bankBump] = await PublicKey.findProgramAddress(
      [Buffer.from("portfolio"), authority.publicKey.toBuffer()],
      program.programId
    );

    pda = pda1;

    await program.rpc.initialize({
      accounts: {
        portfolio: pda,
        authority: authority.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      },
      signers: [authority],
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    assert.equal(
      portfolioAccount.owner.toString(),
      authority.publicKey.toString()
    );
  });

  it("Creates a portfolio", async () => {
    await program.rpc.createPortfolio(bio, {
      accounts: {
        portfolio: pda,
        authority: authority.publicKey,
      },
      signers: [authority],
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    assert.equal(portfolioAccount.bio, bio);
  });

  it("Stores links", async () => {
    await program.rpc.storeLinks(links, {
      accounts: {
        portfolio: pda,
        authority: authority.publicKey,
      },
      signers: [authority],
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    assert.deepEqual(portfolioAccount.links, links);
  });

  it("Stores an image URL", async () => {
    await program.rpc.storeImage(imageUrl, {
      accounts: {
        portfolio: pda,
        authority: authority.publicKey,
      },
      signers: [authority],
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    assert.equal(portfolioAccount.imageUrl, imageUrl);
  });

  it("Requests a vouch for the portfolio", async () => {
    const vouchRequest = {
      vouchedBy: vouchUser,
      comment: vouchComment,
    };

    await program.rpc.requestVouch(vouchRequest, {
      accounts: {
        portfolio: pda,
      },
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    assert.deepEqual(portfolioAccount.vouchRequests[0], vouchRequest);
  });

  it("Approves a vouch for the portfolio", async () => {
    await program.rpc.approveVouch(vouchUser, {
      accounts: {
        portfolio: pda,
        authority: authority.publicKey,
      },
      signers: [authority],
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    const vouches = portfolioAccount.vouches;
    assert.equal(vouches[0].vouchedBy.toString(), vouchUser.toString());
    assert.equal(vouches[0].comment, vouchComment);
  });

  it("Sends a message to the portfolio owner", async () => {
    await program.rpc.sendMessage(messageContent, {
      accounts: {
        portfolio: pda,
        authority: authority.publicKey,
      },
      signers: [authority],
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    const messages = portfolioAccount.messages;
    assert.equal(messages[0].content, messageContent);
    assert.equal(messages[0].sender.toString(), authority.publicKey.toString());
  });

  it("Receives a tip", async () => {
    await program.rpc.tip(tipAmount, {
      accounts: {
        portfolio: pda,
      },
    });

    await delay(2000);

    const portfolioAccount = await program.account.portfolio.fetch(pda);
    assert.equal(portfolioAccount.tipAmount.toString(), tipAmount.toString());
  });
});

function delay(ms: number): Promise<void> {
  return new Promise((resolve) => {
    const start = Date.now();
    let current = start;
    while (current - start < ms) {
      current = Date.now();
    }
    resolve();
  });
}
