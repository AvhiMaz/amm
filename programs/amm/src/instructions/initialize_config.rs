use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeConfigArgs {
    pub seed: u64,
    pub locked: bool,
    pub fee: u16,
}

#[derive(Accounts)]
#[instruction(args: InitializeConfigArgs)]
pub struct InitializeConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
        seeds = [b"config", args.seed.to_le_bytes().as_ref()],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init,
        payer = authority,
        seeds = [b"lp", args.seed.to_le_bytes().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
        mint::token_program = token_program,
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub mint_x: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub mint_y: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl InitializeConfig<'_> {
    pub fn initialize_config(
        ctx: Context<InitializeConfig>,
        args: InitializeConfigArgs,
    ) -> Result<()> {
        ctx.accounts.config.set_inner(Config {
            mint_a: ctx.accounts.mint_x.key(),
            mint_b: ctx.accounts.mint_y.key(),
            bump: ctx.bumps.config,
            lp_bump: ctx.bumps.mint_lp,
            fee: args.fee,
            seed: args.seed,
            authority: ctx.accounts.authority.key(),
            locked: args.locked,
        });

        Ok(())
    }
}
