use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub bump: u8,
    pub lp_bump: u8,
    pub fee: u16,
    pub seed: u64,
    pub authority: Pubkey,
    pub locked: bool,
}
