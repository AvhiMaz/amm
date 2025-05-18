use anchor_lang::prelude::*;

declare_id!("2Zxk1RvazFK2bp2mAwSMdLAPmS36acTHi4B22AzvojYy");

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
