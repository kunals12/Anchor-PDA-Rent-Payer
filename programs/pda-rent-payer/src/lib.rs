use anchor_lang::prelude::*;

declare_id!("G5PvVjVqALUKvNkmPXstGWqD4NtGgzVUzYRQVfebyXfu");

#[program]
pub mod pda_rent_payer {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
