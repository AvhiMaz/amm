#![allow(unexpected_cfgs)]

use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
pub mod state;
pub use state::*;

declare_id!("2Zxk1RvazFK2bp2mAwSMdLAPmS36acTHi4B22AzvojYy");

#[program]
pub mod amm {

    use super::*;

    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        args: InitializeConfigArgs,
    ) -> Result<()> {
        InitializeConfig::initialize_config(ctx, args)
    }
}
