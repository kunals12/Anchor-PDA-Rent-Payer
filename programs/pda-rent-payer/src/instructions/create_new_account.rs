use anchor_lang::{
    prelude::*,
    system_program::{create_account, CreateAccount},
};

#[derive(Accounts)]
pub struct CreateNewAccount<'info> {
    #[account(mut)] // The new account to be created must be mutable.
    new_account: Signer<'info>,
    #[account(
        mut, // `rent_vault` is mutable because it will pay for the account creation.
        seeds = [b"rent_vault"], // The PDA seed for `rent_vault`.
        bump // Bump seed for deriving the PDA address.
    )]
    rent_vault: SystemAccount<'info>, // PDA acting as the funder for the new account.
    system_program: Program<'info, System>, // The System Program used for account creation.
}

// This function creates a new system account and pays for it using the `rent_vault` PDA.
pub fn create_new_account(ctx: Context<CreateNewAccount>) -> Result<()> {
    // PDA signer seeds: Required for the PDA (`rent_vault`) to authorize transactions.
    let signer_seeds: &[&[&[u8]]] = &[&[b"rent_vault", &[ctx.bumps.rent_vault]]];

    // Calculate the minimum lamports required for rent exemption for the new account.
    let lamports = (Rent::get()?).minimum_balance(0);

    // Perform a CPI to the System Program's `create_account` instruction.
    create_account(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(), // Specify the System Program.
            CreateAccount {
                from: ctx.accounts.rent_vault.to_account_info(), // Funder (rent_vault PDA).
                to: ctx.accounts.new_account.to_account_info(),  // New account being created.
            },
        )
        .with_signer(signer_seeds), // Add PDA signer for authorization.
        lamports, // Amount of lamports to transfer for rent exemption.
        0,        // Space allocation for the new account (0 bytes in this case).
        &ctx.accounts.system_program.key(), // Owner of the new account (System Program).
    )?;
    Ok(())
}


/*
Explanation:

Purpose:
The create_new_account function creates a new system-owned account (new_account) funded by the rent_vault PDA.

Key Steps:
Calculate the minimum rent-exempt lamports required for the new account.
Use the System Program's create_account instruction to create and fund the new account.
The rent_vault PDA signs the transaction to authorize the funding using with_signer.

Usage of seeds and bump:
rent_vault is a PDA derived using seeds (b"rent_vault") and its bump.
signer_seeds is passed to with_signer, allowing the PDA to authorize the transfer of lamports.
Space Allocation (0):

The new account is allocated 0 bytes, indicating it's a simple system-owned account without custom data.
*/