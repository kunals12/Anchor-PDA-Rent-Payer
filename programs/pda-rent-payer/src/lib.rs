use anchor_lang::prelude::*;
mod instructions;
use instructions::*;

declare_id!("G5PvVjVqALUKvNkmPXstGWqD4NtGgzVUzYRQVfebyXfu");

#[program]
pub mod pda_rent_payer {
    use super::*;

    pub fn init_rent_vault(ctx: Context<InitRentVault>, amount: u64) -> Result<()> {
        init_rent_vault::init_rent_vault(ctx, amount)
    }

    pub fn create_new_account(ctx: Context<CreateNewAccount>) -> Result<()> {
        create_new_account::create_new_account(ctx)
    }
}
