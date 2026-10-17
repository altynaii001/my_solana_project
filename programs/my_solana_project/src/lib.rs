use anchor_lang::prelude::*;

declare_id!("5YjeKXZ55Vv3kzdiyXyCghmtBMsjmd3FTmLkLkv6MVNJ");

#[program]
pub mod my_solana_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
