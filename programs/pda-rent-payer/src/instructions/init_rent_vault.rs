use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
pub struct InitRentVault<'info> {
    #[account(mut)] // `payer` is mutable because it will transfer lamports.
    pub payer: Signer<'info>,
    #[account(
        mut, // `rent_vault` is mutable as it will receive lamports.
        seeds = [b"rent_vault"], // The PDA seed for `rent_vault`.
        bump // Bump seed for deriving the PDA address.
    )]
    pub rent_vault: SystemAccount<'info>, // The PDA account owned by the System Program.
    system_account: Program<'info, System>, // The System Program, which facilitates transfers.
}

// This function transfers lamports from the payer to the `rent_vault` PDA.
// If the recipient address (`rent_vault`) doesn't exist, a system account is created by default.
pub fn init_rent_vault(ctx: Context<InitRentVault>, amount: u64) -> Result<()> {
    // Perform a CPI (Cross-Program Invocation) to the System Program's `transfer` instruction.
    transfer(
        CpiContext::new(
            ctx.accounts.system_account.to_account_info(), // Specify the System Program.
            Transfer {
                from: ctx.accounts.payer.to_account_info(), // Source of lamports (payer).
                to: ctx.accounts.rent_vault.to_account_info(), // Destination (`rent_vault` PDA).
            },
        ),
        amount, // Amount of lamports to transfer.
    )?;
    Ok(())
}

/*
Explanation:
Purpose:
The init_rent_vault function initializes a PDA (rent_vault) by transferring lamports to it.
If rent_vault doesn't exist, the Solana runtime automatically creates a system-owned account at that address.

Key Steps:
The transfer instruction sends amount lamports from payer to rent_vault.
If the PDA doesn’t exist, a new system account is created at the rent_vault address.

Usage of seeds and bump:
The rent_vault address is derived deterministically using seeds (b"rent_vault") and bump.
This ensures rent_vault always resolves to the same address.
 */
