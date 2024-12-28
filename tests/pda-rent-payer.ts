import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PdaRentPayer } from "../target/types/pda_rent_payer";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
const IDL = require("../target/idl/pda_rent_payer.json");
const programID = new PublicKey(IDL.address);

describe("pda-rent-payer", () => {
  let context;
  let provider: BankrunProvider;
  let program: anchor.Program<PdaRentPayer>;
  let wallet: anchor.Wallet;
  let connection;


  before(async() => {
    context = await startAnchor("", [{name: "pda_rent_payer", programId: programID}], []);
    provider = new BankrunProvider(context);
    program = new anchor.Program<PdaRentPayer>(IDL, provider);
    wallet = provider.wallet as anchor.Wallet;
    connection = provider.connection;
  })

  // PDA for the Rent Vault
  const [rentVaultPDA] = PublicKey.findProgramAddressSync([Buffer.from("rent_vault")], programID);

  it("init rent vault", async () => {
    console.log("wallet public key is", wallet.publicKey);
    
    
    // 1 SOL
    const amount = new anchor.BN(LAMPORTS_PER_SOL);
    // Add your test here.
    const tx = await program.methods.initRentVault(amount)
    .accounts({
      payer: wallet.publicKey,
    })
    .rpc();
    console.log("Your transaction signature", tx);
    // Check rent vault balance
    const accountsInfo = await program.provider.connection.getAccountInfo(rentVaultPDA);
    assert(accountsInfo.lamports === amount.toNumber());
  });
});
